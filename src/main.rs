use std::env;
use std::process::Command;

use console::Style;
use dialoguer::theme::{SelectionStyle, Theme};
use dialoguer::{Input, Select};
use std::fmt;
use strum_macros::{Display, EnumString};

/// A custom theme, built from ColorfulTheme
pub struct CustomTheme {
    /// The style for default values in prompts and similar
    pub defaults_style: Style,
    /// The style for errors indicators
    pub error_style: Style,
    /// The style for user interface indicators
    pub indicator_style: Style,
    /// The style for inactive elements
    pub inactive_style: Style,
    /// The style for active elements
    pub active_style: Style,
    /// The style for values indicating "yes"
    pub yes_style: Style,
    /// The style for values indicating "no"
    pub no_style: Style,
    /// The style for values embedded in prompts
    pub values_style: Style,
}

impl Default for CustomTheme {
    fn default() -> CustomTheme {
        CustomTheme {
            defaults_style: Style::new().dim(),
            error_style: Style::new().red(),
            indicator_style: Style::new().cyan().bold(),
            inactive_style: Style::new().dim(),
            active_style: Style::new(),
            yes_style: Style::new().green(),
            no_style: Style::new().green(),
            values_style: Style::new().cyan(),
        }
    }
}

impl Theme for CustomTheme {
    fn format_prompt(&self, f: &mut fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    fn format_singleline_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "{} [{}]", prompt, self.defaults_style.apply_to(default)),
            None => write!(f, "{}", prompt),
        }
    }

    fn format_error(&self, f: &mut fmt::Write, err: &str) -> fmt::Result {
        write!(f, "{}: {}", self.error_style.apply_to("error"), err)
    }

    fn format_confirmation_prompt(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        match default {
            None => {}
            Some(true) => write!(f, " {} ", self.defaults_style.apply_to("[Y/n]"))?,
            Some(false) => write!(f, " {} ", self.defaults_style.apply_to("[y/N]"))?,
        }
        Ok(())
    }

    fn format_confirmation_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &prompt,
            if selection {
                self.yes_style.apply_to("yes")
            } else {
                self.no_style.apply_to("no")
            }
        )
    }

    fn format_single_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}{}", prompt, self.values_style.apply_to(sel))
    }

    fn format_multi_prompt_selection(
        &self,
        f: &mut fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if idx == 0 { "" } else { ", " },
                self.values_style.apply_to(sel)
            )?;
        }
        Ok(())
    }

    fn format_selection(&self, f: &mut fmt::Write, text: &str, st: SelectionStyle) -> fmt::Result {
        match st {
            SelectionStyle::CheckboxUncheckedSelected => write!(
                f,
                "{} [ ] {}",
                self.indicator_style.apply_to(">"),
                self.active_style.apply_to(text)
            ),
            SelectionStyle::CheckboxUncheckedUnselected => {
                write!(f, "  [ ] {}", self.inactive_style.apply_to(text))
            }
            SelectionStyle::CheckboxCheckedSelected => write!(
                f,
                "{} [{}] {}",
                self.indicator_style.apply_to(">"),
                self.indicator_style.apply_to("x"),
                self.active_style.apply_to(text),
            ),
            SelectionStyle::CheckboxCheckedUnselected => write!(
                f,
                "  [{}] {}",
                self.indicator_style.apply_to("x"),
                self.inactive_style.apply_to(text)
            ),
            SelectionStyle::MenuSelected => write!(
                f,
                "{} {}",
                self.indicator_style.apply_to(">"),
                self.active_style.apply_to(text)
            ),
            SelectionStyle::MenuUnselected => write!(f, "  {}", self.inactive_style.apply_to(text)),
        }
    }
}

#[derive(EnumString, Display)]
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

fn get_attrs(ty: &CommitType) -> CommitTypeDetails {
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
        .map(|e| format!("{}: {}", e, get_attrs(e).description))
        .collect();

    let theme = CustomTheme::default();

    let selection = Select::with_theme(&theme)
        .with_prompt("Select the type of change that you're committing")
        .default(0)
        .items(&m_opts[..])
        .interact_opt()
        .unwrap();
    let s = selection.unwrap();
    let selected_type = &options[s];
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
        if scope.len() > 0 {
            format!("({})", scope)
        } else {
            "".to_owned()
        },
        selected_type_attrs.emoji,
        subject
    );

    let msg_body = if body.len() > 0 {
        format!("\n\n{}", body)
    } else {
        "".to_owned()
    };

    let msg_footer = if breaking.len() > 0 || issues.len() > 0 {
        format!(
            "\n{}{}",
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
    cmd.arg("commit").arg("-m").arg(msg).args(&args[1..]);

    cmd.spawn().expect("failed to run git commit");
}
