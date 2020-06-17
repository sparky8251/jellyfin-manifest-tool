mod manifest;
mod validate;

use std::path::PathBuf;
use structopt::StructOpt;
use validate::validate_manifest;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Jellyfin Manifest Tool",
    about = "A Jellyfin plugin manifest validator/generator"
)]
enum Args {
    #[structopt(about = "Validate a local plugin manifest")]
    Validate {
        #[structopt(parse(from_os_str), help = "The path to manifest you want to validate")]
        file: PathBuf,
    },
}

fn main() {
    match Args::from_args() {
        Args::Validate { file } => validate_manifest(file),
    }
}
