# ghpick

`ghpick` is a simple CLI tool to fetch a file from a GitHub repo.

## Install

You can install it to your `$HOME/.cargo/bin` with `cargo install`

```bash
$ cargo install --git https://github.com/abbit/ghpick
```

## Usage

```bash
$ ghpick abbit/ghpick/src/main.rs
```

```
Usage: ghpick [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to file in repo in format "owner/repo/path/to/file" (example: abbit/ghpick/src/main.rs)

Options:
  -b, --branch <BRANCH>  Branch to fetch from [default: main]
  -d, --dest <DEST>      Destination path to save file [default: .]
  -h, --help             Print help
```
