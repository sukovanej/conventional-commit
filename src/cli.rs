use clap::Parser;

use crate::{commit_message::CommitMessageInput, config::Config, emoji::COMMIT_TYPES};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Milan Suk <Milansuk@email.cz>")]
pub struct Opts {
    #[clap(possible_values=COMMIT_TYPES, index=1, required=true)]
    pub commit_type: String,

    #[clap(short, long, about = "Commit message", required = true)]
    pub message: String,

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
    pub emoji: Option<bool>,

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
    pub fn to_commit_message_input(&self, config: Config) -> CommitMessageInput {
        CommitMessageInput {
            message: self.message.clone(),
            emoji: self.emoji.unwrap_or(config.emoji),
            commit_type: self.commit_type.clone(),
            scope: self.scope.clone(),
            issue: self.issue.clone(),
        }
    }
}
