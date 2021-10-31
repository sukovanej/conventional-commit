mod cli;
mod commit_message;
mod config;
mod emoji;
mod git;

use std::env;

use clap::Parser;

use cli::Opts;
use commit_message::create_commit_message;
use config::{Config, Error};
use git::git_commit;

pub const CONFIG_FILE_NAME: &str = "convetional_commit.toml";

fn load_config() -> Config {
    let pwd = env::current_dir().unwrap();

    let config_file_path = pwd.join(CONFIG_FILE_NAME);
    let config_filename = config_file_path.as_os_str().to_str().unwrap();
    println!("{}", config_filename);

    let maybe_config = Config::load_local(config_filename);

    match maybe_config {
        Err(Error::ParsingError) => panic!("{}", Error::ParsingError),
        Err(Error::ConfigFileNotFound) => Config::default(),
        Ok(config) => config,
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let config = load_config();

    let commit_message = create_commit_message(&opts.to_commit_message_input(config));

    if opts.dry_run {
        println!("{}", commit_message);
    } else {
        git_commit(&commit_message, opts.add_all_files);
    }
}
