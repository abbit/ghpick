use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use tokio::time::{interval, Duration};

const BASE_URL: &str = "https://raw.githubusercontent.com";
const DEFAULT_BRANCH: &str = "main";

#[derive(Parser)]
/// Fetch a file from a GitHub repo
struct Cli {
    /// Branch to fetch from
    #[clap(short, long, default_value = DEFAULT_BRANCH)]
    branch: String,

    /// Destination path to save file
    #[clap(short, long, default_value = ".")]
    dest: PathBuf,

    /// Path to file in repo
    path: String,
}

struct PathParts<'a> {
    full: &'a str,
    filename: &'a str,
}

fn parse_path<'a>(path: &'a str, branch: &'a str) -> Result<PathParts<'a>> {
    let (owner, rest_path) = path
        .split_once('/')
        .ok_or(anyhow::anyhow!("invalid path"))?;
    let (repo, rest_path) = rest_path
        .split_once('/')
        .ok_or(anyhow::anyhow!("invalid path"))?;
    let filename = rest_path
        .split('/')
        .last()
        .ok_or(anyhow::anyhow!("invalid path"))?;
    let full_path = format!("{}/{}/{}/{}", owner, repo, branch, rest_path).leak();

    Ok(PathParts {
        full: full_path,
        filename,
    })
}

async fn fetch_file<'a>(path_parts: &PathParts<'a>) -> Result<String> {
    let url = format!("{}/{}", BASE_URL, path_parts.full);
    let res = reqwest::get(url).await?;
    if !res.status().is_success() {
        let err_msg = format!(
            "failed to fetch {}: {}",
            &path_parts.full,
            res.text().await?
        );
        return Err(anyhow::anyhow!(err_msg));
    }
    let body = res.text().await?;
    Ok(body)
}

async fn save_file<'a>(destpath: &Path, path_parts: &PathParts<'a>, file: &str) -> Result<()> {
    tokio::fs::create_dir_all(destpath).await?;
    tokio::fs::write(destpath.join(path_parts.filename), file).await?;
    Ok(())
}

fn create_spinner() -> indicatif::ProgressBar {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    spinner
}

async fn start_spinner(spinner: &indicatif::ProgressBar, msg: String) {
    let mut intv = interval(Duration::from_millis(120));
    spinner.set_message(msg);
    loop {
        intv.tick().await;
        spinner.tick();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let path_parts = parse_path(&args.path, &args.branch)?;

    let spinner = create_spinner();
    let spin_handle = start_spinner(&spinner, format!("Fetching {}", &args.path));

    let file_downloading = async {
        let file = fetch_file(&path_parts).await?;
        save_file(&args.dest, &path_parts, &file).await?;
        Ok::<(), anyhow::Error>(())
    };

    tokio::select! {
        _ = spin_handle => {},
        download_res = file_downloading => {
            spinner.finish_and_clear();
            match download_res {
                Ok(_) => {
                    spinner.println(format!("{} saved to {}", path_parts.full, args.dest.join(path_parts.filename).display()));
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    println!("Done!");

    Ok(())
}
