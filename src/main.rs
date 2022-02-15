use std::{
    env::{self, temp_dir},
    error::Error,
    process::Command,
};

use regex::Regex;
use uuid::Uuid;

type DynResult<T> = Result<T, Box<dyn Error>>;

fn main() -> DynResult<()> {
    let re = Regex::new(
        r"(?x)
        https://github\.com/                    # The github https address
        (?P<repo>[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+) # The name of the repo
        /pull/                                  # Obligatory /pull/
        (?P<number>\d+)                         # The pull request number
    ",
    )?;

    let args: Vec<_> = env::args().skip(1).filter(|arg| arg != "ghdoc").collect();
    assert_eq!(args.len(), 1);
    let raw_url = args[0].clone();

    let caps = re.captures(&raw_url).expect("Failed to parse URL. Expected the pattern `https://github.com/[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+/pull/\\d+`");

    let (repo, number) = match (caps.name("repo"), caps.name("number")) {
        (Some(repo), Some(number)) => (repo.as_str().to_owned(), number.as_str().to_owned()),
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
        .output()
        .expect("Error cloning repo");
    eprintln!("Cloned {} to {:?}", raw_url, repo_path);

    Command::new("git")
        .current_dir(&repo_path)
        .args(["fetch", "origin", &format!("pull/{}/head:GHDOC", number)])
        .output()
        .expect("Error fetching PR");
    Command::new("git")
        .current_dir(&repo_path)
        .args(["checkout", "GHDOC"])
        .output()
        .expect("Error checking out PR branch");
    eprintln!("Checked out PR {}", number);

    // TODO - bubble up the stderr from this command
    Command::new("cargo")
        .current_dir(&repo_path)
        .args(["doc", "--open"])
        .output()
        .expect("Error generating docs - this may be because the PR doesn't compile or there is an error with the docs.");
    eprintln!("Opened cargo docs");

    Ok(())
}
