// command_task.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A chain of commands with a progress dialog and a result dialog.

use crate::design::dialogs::dialogs_spinner::{IndicatorType, show_spinner_dialog};
use crate::design::dialogs::standard_dialogs::{
    show_dialog_title, show_error_dialog, show_success_dialog,
};
use crate::gettext;
use crate::helper::platform::logging::excerpt;
use gtk4::Widget;
use gtk4::prelude::*;

/// What the user is told when the commands have run successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskOutcome {
    /// Nothing; the progress dialog just closes.
    Nothing,
    /// A success dialog with this message.
    Success(String),
    /// A dialog that shows the output of the commands, with this title.
    Output(String),
}

/// Explains an error that the program knows: returns the explanation for the output of a
/// failed command, or `None`.
pub type ErrorHint = fn(&str) -> Option<String>;

/// A chain of commands that run one after another in the background while a progress dialog
/// is shown. When it ends, the user sees what happened: a success dialog, the output of the
/// commands, or an error dialog.
///
/// The commands run without a shell, and the chain stops at the first one that fails (see
/// [`show_spinner_dialog`]). A program with its own preconditions (a connected device, a
/// chosen folder) checks them and then calls [`run`](Self::run).
///
/// # Examples
///
/// ```no_run
/// use atlbase::{prelude::command_line, ui_prelude::*};
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// CommandTask::new(
///     "Reboot",
///     "Please wait...",
///     vec![command_line("fastboot", ["reboot"])],
/// )
/// .success_message("The device restarts.")
/// .run(&window);
/// ```
#[derive(Debug, Clone)]
pub struct CommandTask {
    title: String,
    message: String,
    commands: Vec<Vec<String>>,
    indicator: IndicatorType,
    outcome: TaskOutcome,
    hint: Option<ErrorHint>,
}

impl CommandTask {
    /// Creates a task. `title` and `message` are shown in the progress dialog (translate them
    /// before).
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        commands: Vec<Vec<String>>,
    ) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            commands,
            indicator: IndicatorType::Spinner,
            outcome: TaskOutcome::Nothing,
            hint: None,
        }
    }

    /// Shows a progress bar instead of a spinner (for long operations such as flashing).
    pub fn progress_bar(mut self) -> Self {
        self.indicator = IndicatorType::ProgressBar;
        self
    }

    /// Tells the user with a success dialog that the task worked.
    pub fn success_message(mut self, message: impl Into<String>) -> Self {
        self.outcome = TaskOutcome::Success(message.into());
        self
    }

    /// Shows the output of the commands in a dialog with this title when the task worked.
    pub fn show_output(mut self, title: impl Into<String>) -> Self {
        self.outcome = TaskOutcome::Output(title.into());
        self
    }

    /// Explains known errors in plain words: if `hint` knows the output of a failed command
    /// it returns an explanation, which is shown above the output.
    pub fn explain_errors(mut self, hint: ErrorHint) -> Self {
        self.hint = Some(hint);
        self
    }

    /// The commands of the task.
    pub fn commands(&self) -> &[Vec<String>] {
        &self.commands
    }

    /// The kind of progress indicator.
    pub fn indicator(&self) -> IndicatorType {
        self.indicator
    }

    /// What the user is told when the task worked.
    pub fn outcome(&self) -> &TaskOutcome {
        &self.outcome
    }

    /// Shows the progress dialog over `parent` and runs the commands in the background. A task
    /// without commands does nothing.
    pub fn run(self, parent: &impl IsA<Widget>) {
        if self.commands.is_empty() {
            return;
        }

        let parent_widget: Widget = parent.upcast_ref::<Widget>().clone();
        let (outcome, hint) = (self.outcome, self.hint);
        show_spinner_dialog(
            parent,
            &self.title,
            &self.message,
            self.commands,
            self.indicator,
            Some(Box::new(move |result| match result {
                Ok(output) => match outcome {
                    TaskOutcome::Nothing => log::debug!("task finished: {}", excerpt(&output)),
                    TaskOutcome::Success(message) => show_success_dialog(&parent_widget, &message),
                    TaskOutcome::Output(title) => {
                        let output = output.trim();
                        let text = if output.is_empty() {
                            gettext!("(no output)")
                        } else {
                            output.to_string()
                        };
                        show_dialog_title(&parent_widget, &title, &text);
                    }
                },
                Err(error) => {
                    log::error!("task failed: {}", excerpt(&error));
                    show_error_dialog(&parent_widget, &task_error_text(&error, hint));
                }
            })),
        );
    }
}

/// The text of the error dialog of a task: the explanation first (if `hint` knows the error),
/// then the output of the failed command.
///
/// # Examples
///
/// ```
/// use atlbase::ui_prelude::task_error_text;
///
/// fn hint(error: &str) -> Option<String> {
///     error.contains("locked").then(|| "The device is locked.".to_string())
/// }
///
/// assert_eq!(
///     task_error_text("FAILED (device is locked)\n", Some(hint)),
///     "The device is locked.\n\nFAILED (device is locked)"
/// );
/// assert_eq!(task_error_text("  just text \n", Some(hint)), "just text");
/// assert_eq!(task_error_text("locked", None), "locked");
/// ```
pub fn task_error_text(error: &str, hint: Option<ErrorHint>) -> String {
    match hint.and_then(|hint| hint(error)) {
        Some(explanation) => format!("{explanation}\n\n{}", error.trim()),
        None => error.trim().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_the_options() {
        let task = CommandTask::new("t", "m", vec![vec!["true".into()]])
            .progress_bar()
            .success_message("done");
        assert_eq!(task.indicator(), IndicatorType::ProgressBar);
        assert_eq!(task.outcome(), &TaskOutcome::Success("done".into()));
        assert_eq!(task.commands().len(), 1);

        let task = task.show_output("Output");
        assert_eq!(task.outcome(), &TaskOutcome::Output("Output".into()));
    }
}
