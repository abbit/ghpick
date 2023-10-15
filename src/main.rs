use std::path::Path;

use anyhow::Result;
use clap::Parser;

const BASE_URL: &str = "https://raw.githubusercontent.com";
const DEFAULT_BRANCH: &str = "main";

#[derive(Parser)]
struct Opts {
    path: String,
    #[clap(short, long, default_value = DEFAULT_BRANCH)]
    branch: String,
    #[clap(short, long, default_value = ".")]
    dest: String,
}

struct PathParts<'a> {
    owner: &'a str,
    repo: &'a str,
    branch: &'a str,
    path: &'a str,
    file_name: &'a str,
}

fn parse_path<'a>(path: &'a str, branch: &'a str) -> Result<PathParts<'a>> {
    let (owner, rest_path) = path
        .split_once('/')
        .ok_or(anyhow::anyhow!("invalid path"))?;
    let (repo, path) = rest_path
        .split_once('/')
        .ok_or(anyhow::anyhow!("invalid path"))?;
    let file_name = path
        .split('/')
        .last()
        .ok_or(anyhow::anyhow!("invalid path"))?;

    Ok(PathParts {
        owner,
        repo,
        branch,
        path,
        file_name,
    })
}

fn get_url<'a>(path_parts: &PathParts<'a>) -> String {
    format!(
        "{}/{}/{}/{}/{}",
        BASE_URL, path_parts.owner, path_parts.repo, path_parts.branch, path_parts.path
    )
}

async fn fetch_file<'a>(path_parts: &PathParts<'a>) -> Result<String> {
    let url = get_url(path_parts);
    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

async fn save_file<'a>(destpath: &Path, path_parts: &PathParts<'a>, file: &str) -> Result<()> {
    tokio::fs::create_dir_all(destpath).await?;
    tokio::fs::write(destpath.join(path_parts.file_name), file).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    let destpath = Path::new(&opts.dest);
    let path_parts = parse_path(&opts.path, &opts.branch)?;

    let file = fetch_file(&path_parts).await?;

    save_file(&destpath, &path_parts, &file).await?;

    Ok(())
}
