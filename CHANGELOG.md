# v0.4.0
- Improved logging using `log`
- Improved CLI handling with `clap`
- Pass through stderr and stdout of git and cargo subprocess' for easier error diagnosis
- Will now generate docs from the top level github repo URL as well
- Will now generate docs for private items unless a CLI flag is set to disallow