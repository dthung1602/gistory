# GISTORY

A tool to create custom commit patterns to display on your GitHub profile

It includes a CLI, a Rust library, a REST API and a webpage

>> [Try out on browser](https://dthung1602.github.io/gistory/) <<
>> [Rust Crate](https://crates.io/crates/gistory) <<

## How it works

- Github updates your commit graph retroactively using the timestamp of the commit itself, not when the commit
  pushed to Github
- Gistory creates a bunch of empty commits with desired timestamp to simulate the commit history of the repo

## Installation

- Install CLI: `cargo install gistory`. The binary `gistory` will be installed to `~/.cargo/bin`
- Install library: `cargo add gistory` in your project
- Webserver: `docker run -p 3000:3000 dthung1602/gistory` or build from source (see below)

## Usage

### CLI

```
gistory [OPTIONS] --start-date <START_DATE> <--full|--random|--pattern-file <PATTERN>|--image <IMAGE>|--text <TEXT>|--raw <RAW_PATERN>>

Options:
  -n, --repo-name <REPOSITORY_NAME>  Repo name. [default: gistory]
  -w, --working-dir <WORKING_DIR>    Working directory. Default to cwd.
  -u, --user-name <USER_NAME>        Git username. Default to global git config.
  -m, --email <EMAIL>                Git user email. This must match GitHub email. Default to global git config.
  -b, --branch <BRANCH_NAME>         Branch name. Default to `master` [default: master]
  -z, --time-zone <TIME_ZONE>        Time zone in format +-HHMM. Defaults to local timezone.
  -s, --start-date <START_DATE>      Start date. Format YYYY-mm-dd.
  -e, --end-date <END_DATE>          End date. Format YYYY-mm-dd.
  -c, --commit-count <COMMIT_COUNT>  Commit count [default: many] [possible values: zero, few, some, many, a-lot]
      --font <FONT>                  Font for text [default: subway-tracker] [possible values: subway-tracker]
  -f, --full                         Fill all days with the same number of commits
  -r, --random                       Fill all days with random number of commits
  -p, --pattern-file <PATTERN>       Draw pattern from file. File format: text file contains character from 0->4 on less than 7 lines. 0 means no commit, 4 means lots of commits
  -i, --image <IMAGE>                Draw image. Image will be re-scaled to 7-pixel height and turned to grayscale
  -t, --text <TEXT>                  Print given text on grid
  -x, --raw <RAW_PATERN>             Raw commit count pattern with characters 0->4. Invalid characters are considered as 0
  -h, --help                         Print help
  -V, --version                      Print version
```

### Library

```rust 
use std::path::PathBuf;
use std::str::FromStr;

use chrono::NaiveDate;
use gistory::git::repo::Repo;
use gistory::visualizer::{CommitGrid, CommitCount, Font};

async fn main() {
  let start_date = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
  let end_date = NaiveDate::parse_from_str("2023-05-05", "%Y-%m-%d").unwrap();

  let mut grid = CommitGrid::new(start_date);

  // Use ONE of these methods to populate the grid
  grid.full(CommitCount::Many, end_date).unwrap();
  grid.read_pattern_file("pattern.txt").await.unwrap();
  grid.read_image_file("img.png").await.unwrap();
  grid.show_text(
    "Some Text!".to_string(),
    Font::SubwayTracker,
    CommitCount::Few,
  )
          .unwrap();
  grid.set_data(vec![CommitCount::Few, CommitCount::ALot, CommitCount::Some]);

  // Specify repo details
  let mut repo = Repo::new(
    PathBuf::from("my_repo"),                        // repo path
    "master".to_string(),                            // branch
    chrono::FixedOffset::from_str("+0700").unwrap(), // timezone
    "username".to_string(),                          // username
    "abc@example.com".to_string(),                   // email
  );

  // Initialize the repo and populate it with the grid
  repo.init().await.unwrap();
  grid.populate_repo(&mut repo).await.unwrap();
}
```

## Project structure

- `gistory` - Rust CLI & library, contain the core logic
- `webserver` - Axum webserver, serving the REST API
- `frontend` - React webpage to interact with the API

## Development

### CLI & Library
```shell
cd gistory
cargo build
cargo run -- --help
```

### Webserver

Using docker:

Pull existing image: `docker run -p 3000:3000 dthung1602/gistory`, or build from source:

```shell
docker build -t gistory .
docker run -p 3000:3000 gistory
```

Without docker:

- First install [diesel-cli](https://diesel.rs/guides/getting-started.html#installing-diesel-cli)
- Make sure you have `libsqlite3-dev` installed on your system.
- Run the following commands:
```shell
# run database migrations
cd webserver
export DATABASE_URL='./database.db' # or save it to .env file
diesel migration run
# run webserver
cargo build
cargo run --bin webserver # webserver will be running on http://localhost:3000
```

### Frontend

Make sure you have `node 22` installed on your system.

```shell
npm install
npm run dev
```

Set environment variable `VITE_BACKEND_ENDPOINT` to the URL of the webserver, default to `http://localhost:3000/api/`.

To publish the frontend to Github pages, create a Github token, use it as password in `npm run deploy`
