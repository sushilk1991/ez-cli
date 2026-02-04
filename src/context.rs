use std::io::IsTerminal;

/// Global state carrier passed to every command.
pub struct CommandContext {
    pub json: bool,
    pub yes: bool,
    pub dry_run: bool,
    #[allow(dead_code)]
    pub is_tty: bool,
    pub is_stdin_tty: bool,
}

impl CommandContext {
    pub fn new(json: bool, yes: bool, dry_run: bool) -> Self {
        Self {
            json,
            yes,
            dry_run,
            is_tty: std::io::stdout().is_terminal(),
            is_stdin_tty: std::io::stdin().is_terminal(),
        }
    }

    /// Whether to prompt for confirmation.
    /// Returns false if --yes was passed or stdin is not a TTY.
    pub fn should_confirm(&self) -> bool {
        !self.yes && self.is_stdin_tty
    }
}
