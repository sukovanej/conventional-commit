mod cli;
mod commit_message;
mod emoji;
mod git;

use clap::Parser;

use cli::Opts;
use commit_message::create_commit_message;
use git::git_commit;

fn main() {
    let opts: Opts = Opts::parse();
    let commit_message = create_commit_message(&opts.to_commit_message_input());

    if opts.dry_run {
        println!("{}", commit_message);
    } else {
        git_commit(&commit_message, opts.add_all_files);
    }
}
