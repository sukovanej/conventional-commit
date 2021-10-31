pub const COMMIT_TYPES: [&str; 11] = [
    "build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "revert", "style", "test",
];

pub fn commit_type_to_emoji(commit_type: &str) -> &'static str {
    match commit_type {
        "build" => ":construction_worker:",
        "chore" => ":gear:",
        "ci" => ":rocket:",
        "docs" => ":books:",
        "feat" => ":sparkles:",
        "fix" => ":bug:",
        "perf" => ":racehorse:",
        "refactor" => ":hammer:",
        "revert" => ":rewind:",
        "style" => ":nail_care:",
        "test" => ":rotating_light:",
        _ => panic!("unknown commit type"),
    }
}
