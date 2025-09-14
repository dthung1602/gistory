mod error;
mod git;
mod utils;
mod visualizer;

use std::path::PathBuf;

use clap::{ArgAction, ArgGroup, Command, arg, value_parser};
use log::debug;

use crate::visualizer::CommitCount;

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();

    let cwd = std::env::current_dir()?.into_os_string();
    let local_tz = chrono::Local::now().format("%z").to_string();
    let default_username = git::utils::get_global_config("user.name")?;
    let default_email = git::utils::get_global_config("user.email")?;

    let matches = Command::new("gistory")
        .version("0.1.0")
        .about("A tool to draw on GitHub commit graph")
        .arg(
            arg!(-n --"repo-name" <REPOSITORY_NAME> "Repo name.").default_value("gistory")
        )
        .arg(
            arg!(-w --"working-dir" <WORKING_DIR> "Working directory. Default to cwd.")
                .value_parser(value_parser!(PathBuf))
                .default_value(cwd)
        )
        .arg(
            arg!(-u --"user-name" <USER_NAME> "Git username. Default to global git config.")
                .default_value(default_username)
        )
        .arg(
            arg!(-m --"email" <EMAIL> "Git user email. This must match GitHub email. Default to global git config.")
                .default_value(default_email)
        )
        .arg(
            arg!(-b --"branch" <BRANCH_NAME> "Branch name. Default to `master`")
                .default_value("master")
        )
        .arg(
            arg!(-z --"time-zone" <TIME_ZONE> "Time zone in format +-HHMM. Defaults to local timezone.")
                .value_parser(value_parser!(chrono::FixedOffset))
                .default_value(local_tz)
        )
        .arg(
            arg!(-s --"start-date" <START_DATE> "Start date. Format YYYY-mm-dd.").required(true)
                .value_parser(value_parser!(chrono::NaiveDate))
        )
        .arg(
            arg!(-e --"end-date" <END_DATE> "End date. Format YYYY-mm-dd.").required(false)
                .value_parser(value_parser!(chrono::NaiveDate))
                .required_if_eq_any([
                    ("full", "true"),
                    ("random", "true")
                ])
        )
        .arg(
            arg!(-c --"commit-count" <COMMIT_COUNT> "Commit count").required(false)
                .value_parser(value_parser!(visualizer::CommitCount))
                .default_value("many")
        )
        .arg(
            arg!(--"font" <FONT> "Font for text").required(false)
                .value_parser(value_parser!(visualizer::Font))
                .default_value("subway-tracker")
        )
        .arg(
            arg!(-f --"full" "Fill all days with the same number of commits").action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-r --"random" "Fill all days with random number of commits").action(ArgAction::SetTrue)
        )
        .arg(
            arg!(-p --"pattern-file" <PATTERN> "Draw pattern from file. File format: text file contains character from 0->4 on less than 7 lines. 0 means no commit, 4 means lots of commits")
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-i --"image" <IMAGE> "Draw image. Image will be re-scaled to 7-pixel height and turned to grayscale")
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-t --"text" <TEXT> "Print given text on grid")
        )
        .arg(
            arg!(-x --"raw" <RAW_PATERN> "Raw commit count pattern with characters 0->4. Invalid characters are considered as 0")
        )
        .group(
            ArgGroup::new("method").args(["full", "random", "pattern-file", "image", "text", "raw"]).required(true).multiple(false)
        )
        .get_matches();

    let start_date = matches.get_one::<chrono::NaiveDate>("start-date").unwrap();
    let mut grid = visualizer::CommitGrid::new(*start_date);

    if matches.get_flag("full") {
        let end_date = matches.get_one::<chrono::NaiveDate>("end-date").unwrap();
        let commit_count = matches
            .get_one::<visualizer::CommitCount>("commit-count")
            .unwrap();
        grid.full(*commit_count, *end_date)?;
    } else if matches.get_flag("random") {
        let end_date = matches.get_one::<chrono::NaiveDate>("end-date").unwrap();
        grid.random(*end_date)?;
    } else if matches.contains_id("pattern-file") {
        let pattern_file = matches.get_one::<PathBuf>("pattern-file").unwrap();
        grid.read_pattern_file(pattern_file).await?;
    } else if matches.contains_id("image") {
        let image = matches.get_one::<PathBuf>("image").unwrap();
        grid.read_image_file(image).await?;
    } else if matches.contains_id("text") {
        let text = matches.get_one::<String>("text").unwrap();
        let font = matches.get_one::<visualizer::Font>("font").unwrap();
        let commit_count = matches
            .get_one::<visualizer::CommitCount>("commit-count")
            .unwrap();
        grid.show_text(text.clone(), *font, *commit_count)?;
    } else if matches.contains_id("raw") {
        let raw_pattern = matches.get_one::<String>("raw").unwrap();
        let commit_counts: Vec<CommitCount> = raw_pattern.chars().map(|c| c.into()).collect();
        grid.set_data(commit_counts);
    } else {
        unreachable!("No method flag provided");
    };
    debug!("Grid: {grid:?}");

    let working_dir = matches.get_one::<PathBuf>("working-dir").unwrap();
    let repo_name = matches.get_one::<String>("repo-name").unwrap();
    let repo_path = working_dir.join(repo_name);

    let branch = matches.get_one::<String>("branch").unwrap();
    let time_zone = matches.get_one::<chrono::FixedOffset>("time-zone").unwrap();
    let user_name = matches.get_one::<String>("user-name").unwrap();
    let email = matches.get_one::<String>("email").unwrap();

    let mut repo = git::repo::Repo::new(
        repo_path,
        branch.clone(),
        *time_zone,
        user_name.clone(),
        email.clone(),
    );
    debug!("Repo: {repo:?}");
    repo.init().await?;
    grid.populate_repo(&mut repo).await?;

    debug!("Done");
    Ok(())
}
