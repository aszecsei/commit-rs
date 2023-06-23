#![warn(clippy::pedantic)]

use std::env;
use std::process::{Command, Stdio};

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use strum_macros::{Display, EnumString};

use textwrap::fill;

#[derive(Copy, Clone, EnumString, Display, PartialEq)]
enum CommitType {
    #[strum(serialize = "test")]
    Test,
    #[strum(serialize = "feat")]
    Feature,
    #[strum(serialize = "fix")]
    Fix,
    #[strum(serialize = "chore")]
    Chore,
    #[strum(serialize = "docs")]
    Docs,
    #[strum(serialize = "refactor")]
    Refactor,
    #[strum(serialize = "release")]
    Release,
    #[strum(serialize = "style")]
    Style,
    #[strum(serialize = "ci")]
    CI,
    #[strum(serialize = "perf")]
    Perf,
}

struct CommitTypeDetails {
    description: String,
    emoji: String,
}

fn get_attrs(ty: CommitType) -> CommitTypeDetails {
    match ty {
        CommitType::Chore => CommitTypeDetails {
            description: "Build process or auxiliary tool changes".to_owned(),
            emoji: "🤖".to_owned(),
        },
        CommitType::CI => CommitTypeDetails {
            description: "CI related changes".to_owned(),
            emoji: "🎡".to_owned(),
        },
        CommitType::Docs => CommitTypeDetails {
            description: "Documentation only changes".to_owned(),
            emoji: "✏️".to_owned(),
        },
        CommitType::Feature => CommitTypeDetails {
            description: "A new feature".to_owned(),
            emoji: "🎸".to_owned(),
        },
        CommitType::Fix => CommitTypeDetails {
            description: "A bug fix".to_owned(),
            emoji: "🐛".to_owned(),
        },
        CommitType::Perf => CommitTypeDetails {
            description: "A code change that improves performance".to_owned(),
            emoji: "⚡️".to_owned(),
        },
        CommitType::Refactor => CommitTypeDetails {
            description: "A code change that neither fixes a bug or adds a feature".to_owned(),
            emoji: "💡".to_owned(),
        },
        CommitType::Release => CommitTypeDetails {
            description: "Create a release commit".to_owned(),
            emoji: "🏹".to_owned(),
        },
        CommitType::Style => CommitTypeDetails {
            description: "Markup, white-space, formatting, missing semi-colons...".to_owned(),
            emoji: "💄".to_owned(),
        },
        CommitType::Test => CommitTypeDetails {
            description: "Adding missing tests".to_owned(),
            emoji: "💍".to_owned(),
        },
    }
}

fn main() {
    println!("\nAll commit message lines will be cropped at 100 characters.\n");

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
    let m_opts: Vec<String> = options
        .iter()
        .map(|&e| format!("{}: {}", e, get_attrs(e).description))
        .collect();

    let theme = ColorfulTheme::default();

    let selection = Select::with_theme(&theme)
        .with_prompt("Select the type of change that you're committing")
        .default(0)
        .items(&m_opts[..])
        .interact_opt()
        .unwrap();
    let s = selection.unwrap();
    let selected_type = options[s];
    let selected_type_attrs = get_attrs(selected_type);

    let scope = Input::<String>::with_theme(&theme)
        .with_prompt("What is the scope of this change (e.g. component or file name)? (press enter to skip) ")
        .allow_empty(true)
        .interact().unwrap();
    let subject = Input::<String>::with_theme(&theme)
        .with_prompt("Write a short, imperative tense description of the change:\n")
        .interact()
        .unwrap();
    let body = Input::<String>::with_theme(&theme)
        .with_prompt("Provide a longer description of the change: (press enter to skip)\n")
        .allow_empty(true)
        .interact()
        .unwrap();

    let breaking = Input::<String>::with_theme(&theme)
        .with_prompt("Describe any breaking changes: (press enter to skip) ")
        .allow_empty(true)
        .interact()
        .unwrap();
    let issues = Input::<String>::with_theme(&theme)
        .with_prompt("Related issues: (press enter to skip) ")
        .allow_empty(true)
        .interact()
        .unwrap();

    // Generate commit message
    let msg_header = format!(
        "{}{}: {} {}",
        selected_type,
        if scope.is_empty() {
            String::new()
        } else {
            format!("({scope})")
        },
        selected_type_attrs.emoji,
        subject
    );

    let msg_body = if body.is_empty() {
        String::new()
    } else {
        format!("\n\n{body}")
    };

    let msg_footer = if !breaking.is_empty() || !issues.is_empty() {
        format!(
            "\n{}{}",
            if breaking.is_empty() {
                String::new()
            } else {
                format!("\nBREAKING CHANGE: {breaking}")
            },
            if issues.is_empty() {
                String::new()
            } else {
                format!("\nRelated issues: {issues}")
                
            }
        )
    } else {
        String::new()
    };

    let msg_header_capped: String = msg_header.chars().take(100).collect();
    let msg_body_wrapped = fill(&msg_body, 100);
    let msg_footer_wrapped = fill(&msg_footer, 100);

    let msg = format!(
        "{msg_header_capped}{msg_body_wrapped}{msg_footer_wrapped}"
    );

    let args: Vec<String> = env::args().collect();
    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("-m").arg(msg).args(&args[1..]);

    cmd.stdout(Stdio::null())
        .spawn()
        .expect("failed to run git commit");
}
