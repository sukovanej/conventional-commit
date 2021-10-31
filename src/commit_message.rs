use crate::emoji::commit_type_to_emoji;

pub struct CommitMessageInput {
    pub message: String,
    pub emoji: bool,
    pub commit_type: String,
    pub scope: Option<String>,
    pub issue: Option<String>,
}

pub fn create_commit_message(opts: &CommitMessageInput) -> String {
    let mut commit_message = String::new();

    commit_message += if opts.emoji {
        commit_type_to_emoji(&opts.commit_type)
    } else {
        &opts.commit_type
    };

    commit_message += match (&opts.scope, opts.emoji) {
        (Some(scope), false) => format!("({}):", scope),
        (Some(scope), true) => format!(" {}:", scope),
        (None, false) => ":".to_string(),
        (None, true) => "".to_string(),
    }.as_str();

    commit_message += format!(" {}", &opts.message).as_str();

    if let Some(issue) = &opts.issue {
        commit_message += format!(" ({})", issue).as_str();
    }

    commit_message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_commit_message() {
        let message = "message".to_string();
        let commit_type = "feat".to_string();

        // (scope, issue, emoji)
        let parametrize: Vec<(Option<&str>, Option<&str>, bool, String)> = vec![
            (None, None, false, format!("{}: {}", commit_type, message)),
            (
                Some("scope"),
                None,
                false,
                format!("{}(scope): {}", commit_type, message),
            ),
            (
                None,
                Some("issue"),
                false,
                format!("{}: {} (issue)", commit_type, message),
            ),
            (None, None, true, format!(":sparkles: {}", message)),
            (
                Some("scope"),
                Some("issue"),
                false,
                format!("{}(scope): {} (issue)", commit_type, message),
            ),
            (
                None,
                Some("issue"),
                true,
                format!(":sparkles: {} (issue)", message),
            ),
            (
                Some("scope"),
                None,
                true,
                format!(":sparkles: scope: {}", message),
            ),
            (
                Some("scope"),
                Some("issue"),
                true,
                format!(":sparkles: scope: {} (issue)", message),
            ),
        ];

        for (scope, issue, emoji, expected) in parametrize.iter() {
            let opts = CommitMessageInput {
                message: message.clone(),
                commit_type: commit_type.clone(),
                scope: scope.map(|i| i.to_string()),
                issue: issue.map(|i| i.to_string()),
                emoji: *emoji,
            };
            assert_eq!(&create_commit_message(&opts), expected);
        }
    }
}
