use std::process::Command;
use std::env;

use dialoguer::{theme::ColorfulTheme, Select, Input};
use strum_macros::{Display, EnumString, EnumIter};

#[derive(EnumString, Display)]
enum CommitType {
    #[strum(serialize="test")]
    Test,
    #[strum(serialize="feat")]
    Feature,
    #[strum(serialize="fix")]
    Fix,
    #[strum(serialize="chore")]
    Chore,
    #[strum(serialize="docs")]
    Docs,
    #[strum(serialize="refactor")]
    Refactor,
    #[strum(serialize="release")]
    Release,
    #[strum(serialize="style")]
    Style,
    #[strum(serialize="ci")]
    CI,
    #[strum(serialize="perf")]
    Perf,
}

struct CommitTypeDetails {
    description: String,
    emoji: String,
    value: CommitType,
}

fn get_attrs(ty: &CommitType) -> CommitTypeDetails {
    match ty {
        CommitType::Chore => CommitTypeDetails {
            description: "Build process or auxiliary tool changes".to_owned(),
            emoji: "🤖".to_owned(),
            value: CommitType::Chore
        },
        CommitType::CI => CommitTypeDetails {
            description: "CI related changes".to_owned(),
            emoji: "🎡".to_owned(),
            value: CommitType::CI,
        },
        CommitType::Docs => CommitTypeDetails {
            description: "Documentation only changes".to_owned(),
            emoji: "✏️".to_owned(),
            value: CommitType::Docs,
        },
        CommitType::Feature => CommitTypeDetails {
            description: "A new feature".to_owned(),
            emoji: "🎸".to_owned(),
            value: CommitType::Feature,
        },
        CommitType::Fix => CommitTypeDetails {
            description: "A bug fix".to_owned(),
            emoji: "🐛".to_owned(),
            value: CommitType::Fix,
        },
        CommitType::Perf => CommitTypeDetails {
            description: "A code change that improves performance".to_owned(),
            emoji: "⚡️".to_owned(),
            value: CommitType::Perf,
        },
        CommitType::Refactor => CommitTypeDetails {
            description: "A code change that neither fixes a bug or adds a feature".to_owned(),
            emoji: "💡".to_owned(),
            value: CommitType::Refactor,
        },
        CommitType::Release => CommitTypeDetails {
            description: "Create a release commit".to_owned(),
            emoji: "🏹".to_owned(),
            value: CommitType::Release,
        },
        CommitType::Style => CommitTypeDetails {
            description: "Markup, white-space, formatting, missing semi-colons...".to_owned(),
            emoji: "💄".to_owned(),
            value: CommitType::Style,
        },
        CommitType::Test => CommitTypeDetails {
            description: "Adding missing tests".to_owned(),
            emoji: "💍".to_owned(),
            value: CommitType::Test
        },
    }
}

fn main() {

    println!("\nAll commit message lines will be cropped at 100 characters.\n");

    let mut selection = 0;
    // TODO: Change this to EnumIter
    let options = vec![
        CommitType::Feature,
        CommitType::Fix,
        CommitType::Docs,
        CommitType::Style,
        CommitType::Refactor,
        CommitType::Perf,
        CommitType::Test,
        CommitType::Chore,
    ];
    let m_opts: Vec<String> = options.iter().map(|e| format!("{}: {}", e, get_attrs(e).description)).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of change that you're committing")
        .default(0)
        .items(&m_opts[..])
        .interact_opt()
        .unwrap();
    let s = selection.unwrap();
    let selected_type = &options[s];
    let selected_type_attrs = get_attrs(selected_type);

    let scope = Input::<String>::new()
        .with_prompt("What is the scope of this change (e.g. component or file name)? (press enter to skip)")
        .allow_empty(true)
        .interact().unwrap();
    let subject = Input::<String>::new()
        .with_prompt("Write a short, imperative tense description of the change:\n")
        .interact().unwrap();
    let body = Input::<String>::new()
        .with_prompt("Provide a longer description of the change: (press enter to skip)\n")
        .allow_empty(true)
        .interact().unwrap();

    let breaking = Input::<String>::new()
        .with_prompt("Describe any breaking changes: (press enter to skip)")
        .allow_empty(true)
        .interact().unwrap();
    let issues = Input::<String>::new()
        .with_prompt("Related issues: (press enter to skip)")
        .allow_empty(true)
        .interact().unwrap();

    // Generate commit message
    let msg_header = format!("{}{}: {} {}",
        selected_type,
        if scope.len() > 0 { format!("({})", scope) } else { "".to_owned() },
        selected_type_attrs.emoji,
        subject
    );

    let msg_body = if body.len() > 0 { format!("\n\n{}", body) } else { "".to_owned() };

    let msg_footer = if breaking.len() > 0 || issues.len() > 0 {
        format!("\n{}{}",
            if breaking.len() > 0 {
                format!("\nBREAKING CHANGE: {}", breaking)
            } else {
                "".to_owned()
            },
            if issues.len() > 0 {
                format!("\nRelated issues: {}", issues)
            } else {
                "".to_owned()
            }
        )
    } else {
        "".to_owned()
    };

    let msg = format!("{}{}{}", msg_header, msg_body, msg_footer);

    let args: Vec<String> = env::args().collect();
    let mut cmd = Command::new("git");
    cmd.arg("commit")
        .arg(format!("-m \"{}\"", msg))
        .args(&args[1..]);

    cmd.spawn()
        .expect("failed to run git commit");
}
