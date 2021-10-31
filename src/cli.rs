use clap::Parser;

use crate::{commit_message::CommitMessageInput, emoji::COMMIT_TYPES};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Milan Suk <Milansuk@email.cz>")]
pub struct Opts {
    #[clap(short, long)]
    pub message: String,

    #[clap(short='t', long, possible_values=COMMIT_TYPES)]
    pub commit_type: String,

    #[clap(short, long)]
    pub scope: Option<String>,

    #[clap(short, long)]
    pub issue: Option<String>,

    #[clap(short, long, takes_value = false)]
    pub emoji: bool,

    #[clap(short, long, takes_value = false)]
    pub add_all_files: bool,

    #[clap(short, long, takes_value = false)]
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
