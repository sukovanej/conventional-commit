use clap::Parser;

use crate::{commit_message::CommitMessageInput, emoji::COMMIT_TYPES};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Milan Suk <Milansuk@email.cz>")]
pub struct Opts {
    #[clap(short, long, about = "Commit message")]
    pub message: String,

    #[clap(short='t', long, possible_values=COMMIT_TYPES)]
    pub commit_type: String,

    #[clap(
        short,
        long,
        about = "Optional information specifying the section of the codebase"
    )]
    pub scope: Option<String>,

    #[clap(
        short,
        long,
        about = "Identifier of the github issue / jira issue / ..."
    )]
    pub issue: Option<String>,

    #[clap(
        short,
        long,
        takes_value = false,
        about = "If turned on, an emoji will be used instead of name of the commit type"
    )]
    pub emoji: bool,

    #[clap(
        short,
        long,
        takes_value = false,
        about = "-a option for the git commit command"
    )]
    pub add_all_files: bool,

    #[clap(
        short,
        long,
        takes_value = false,
        about = "Don't do the commit, only show the commit message"
    )]
    pub dry_run: bool,
}

impl Opts {
    pub fn to_commit_message_input(&self) -> CommitMessageInput {
        CommitMessageInput {
            message: self.message.clone(),
            emoji: self.emoji,
            commit_type: self.commit_type.clone(),
            scope: self.scope.clone(),
            issue: self.issue.clone(),
        }
    }
}
