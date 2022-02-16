use std::{
    env::{self, temp_dir},
    error::Error,
    process::{Command, Stdio},
};

use clap::Parser;
use env_logger::Env;
use log::info;
use regex::Regex;
use uuid::Uuid;

type DynResult<T> = Result<T, Box<dyn Error>>;

/// Generate the Rust docs for a Github PR or repo.
///
/// This tool relies entirely on the `git` and `cargo` binaries, which must be in your path.
/// Additionally, you must have set up SSH credentials and have permission to clone the repo
/// you are trying to generate docs for.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Opts {
    /// The URL to the Pull request or repo.
    ///
    /// This can either by a pull request url, i.e. https://github.com/fulcrumgenomics/cargo-ghdoc/pull/1
    /// Or a plain path to a repo, i.e. https://github.com/fulcrumgenomics/cargo-ghdoc
    url: String,
}

fn main() -> DynResult<()> {
    let opts = setup();

    let re = Regex::new(
        r"(?x)
        https://github\.com/                    # The github https address
        (?P<repo>[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+) # The name of the repo
        (                                       # Optional match portion if this is a pull request
            /pull/                              # Obligatory /pull/ to designate a pull request
            (?P<number>\d+)                     # The pull request number
        )?                                 
    ",
    )?;

    let raw_url = opts.url;

    let caps = re.captures(&raw_url).expect("Failed to parse URL. Expected the pattern `https://github.com/[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+/pull/\\d+`");

    let (repo, number) = match (caps.name("repo"), caps.name("number")) {
        (Some(repo), Some(number)) => (repo.as_str().to_owned(), Some(number.as_str().to_owned())),
        (Some(repo), None) => (repo.as_str().to_owned(), None),
        _ => panic!("Failed to parse the `username/repo` or the PR number from the provided URL"),
    };

    let dir = temp_dir();
    let uuid = Uuid::new_v4();
    let repo_path = dir.join(uuid.to_string());

    Command::new("git")
        .args([
            "clone",
            &format!("git@github.com:{}", &repo),
            repo_path.as_os_str().to_str().unwrap(),
        ])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Error cloning repo");
    info!("Cloned {} to {:?}", raw_url, repo_path);

    if let Some(number) = number {
        Command::new("git")
            .current_dir(&repo_path)
            .args(["fetch", "origin", &format!("pull/{}/head:GHDOC", number)])
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output()
            .expect("Error fetching PR");
        Command::new("git")
            .current_dir(&repo_path)
            .args(["checkout", "GHDOC"])
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output()
            .expect("Error checking out PR branch");
        info!("Checked out PR {}", number);
    }

    info!("Generating docs:");
    Command::new("cargo")
        .current_dir(&repo_path)
        .args(["doc", "--open"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Error generating docs - this may be because the PR doesn't compile or there is an error with the docs.");

    Ok(())
}

/// Parse args and set up logging
fn setup() -> Opts {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Handle the case where this is invoked as a cargo subcommand, which adds an extra positional argument.
    let args = env::args().filter(|x| x != "ghdoc");

    Opts::parse_from(args)
}
