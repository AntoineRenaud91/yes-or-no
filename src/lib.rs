//! # yes-or-no
//! A very simple yes or no CLI tool!
//!
//! ## Installation
//!
//! Add `yes-or-no` to your project's dependencies:
//! ```toml
//! [dependencies]
//! yes-or-no = "0.1"
//! ```
//!
//! ## Usage:
//! Here is an example of how to use `yes-or-no` in your Rust project:
//! ```rust,no_run
//! use yes_or_no::yes_or_no;
//!
//! fn main() -> std::io::Result<()> {
//!     match yes_or_no("🦀 Do you like Rust? 🦀", true)? {
//!         Some(true) => println!("You like Rust! 🤩"),
//!         Some(false) => println!("You don't like Rust... 😭"),
//!         None => println!("Cancelled."),
//!     }
//!     Ok(())
//! }
//! ```
//! When you run the program, you'll be presented with a prompt in your terminal:
//! ```text
//! Do you like Rust? Yes [✓] No [ ]
//! ```
//! - Navigate between "Yes" and "No" using the left and right arrow keys.
//! - Press Enter to select your choice.
//! - Press Escape to select "No".
//! - Press Ctrl+C to cancel (returns `None`).

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

/// Represents the user's selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Yes,
    No,
}

impl Selection {
    fn as_bool(self) -> bool {
        matches!(self, Selection::Yes)
    }
}

impl From<bool> for Selection {
    fn from(value: bool) -> Self {
        if value {
            Selection::Yes
        } else {
            Selection::No
        }
    }
}

/// Result of processing a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    /// User confirmed their selection
    Confirm(Selection),
    /// User cancelled (Ctrl+C)
    Cancel,
    /// Selection changed, continue prompting
    Continue(Selection),
}

/// Process a key event and return the resulting action.
/// This is the pure logic extracted for testability.
pub fn process_key(key_code: KeyCode, modifiers: KeyModifiers, current: Selection) -> KeyAction {
    match key_code {
        KeyCode::Left => KeyAction::Continue(Selection::Yes),
        KeyCode::Right => KeyAction::Continue(Selection::No),
        KeyCode::Enter => KeyAction::Confirm(current),
        KeyCode::Esc => KeyAction::Confirm(Selection::No),
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => KeyAction::Cancel,
        _ => KeyAction::Continue(current),
    }
}

/// RAII guard that restores terminal state on drop.
struct TerminalGuard;

impl TerminalGuard {
    fn new(stdout: &mut io::Stdout) -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

/// Displays an interactive yes/no prompt and returns the user's choice.
///
/// # Arguments
/// * `question` - The question to display to the user
/// * `is_yes_initial` - Whether "Yes" should be initially selected
///
/// # Returns
/// * `Ok(Some(true))` - User selected "Yes"
/// * `Ok(Some(false))` - User selected "No" (or pressed Escape)
/// * `Ok(None)` - User cancelled with Ctrl+C
/// * `Err(_)` - A terminal I/O error occurred
pub fn yes_or_no(question: &str, is_yes_initial: bool) -> io::Result<Option<bool>> {
    let mut stdout = io::stdout();
    let _guard = TerminalGuard::new(&mut stdout)?;
    let mut selection = Selection::from(is_yes_initial);

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;

        match selection {
            Selection::Yes => print!("{question} Yes [✓] No [ ]"),
            Selection::No => print!("{question} Yes [ ] No [✓]"),
        }
        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match process_key(key_event.code, key_event.modifiers, selection) {
                KeyAction::Confirm(sel) => return Ok(Some(sel.as_bool())),
                KeyAction::Cancel => return Ok(None),
                KeyAction::Continue(sel) => selection = sel,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_from_bool() {
        assert_eq!(Selection::from(true), Selection::Yes);
        assert_eq!(Selection::from(false), Selection::No);
    }

    #[test]
    fn test_selection_as_bool() {
        assert!(Selection::Yes.as_bool());
        assert!(!Selection::No.as_bool());
    }

    #[test]
    fn test_process_key_left_selects_yes() {
        let action = process_key(KeyCode::Left, KeyModifiers::empty(), Selection::No);
        assert_eq!(action, KeyAction::Continue(Selection::Yes));
    }

    #[test]
    fn test_process_key_right_selects_no() {
        let action = process_key(KeyCode::Right, KeyModifiers::empty(), Selection::Yes);
        assert_eq!(action, KeyAction::Continue(Selection::No));
    }

    #[test]
    fn test_process_key_enter_confirms_current() {
        let action = process_key(KeyCode::Enter, KeyModifiers::empty(), Selection::Yes);
        assert_eq!(action, KeyAction::Confirm(Selection::Yes));

        let action = process_key(KeyCode::Enter, KeyModifiers::empty(), Selection::No);
        assert_eq!(action, KeyAction::Confirm(Selection::No));
    }

    #[test]
    fn test_process_key_escape_confirms_no() {
        let action = process_key(KeyCode::Esc, KeyModifiers::empty(), Selection::Yes);
        assert_eq!(action, KeyAction::Confirm(Selection::No));
    }

    #[test]
    fn test_process_key_ctrl_c_cancels() {
        let action = process_key(KeyCode::Char('c'), KeyModifiers::CONTROL, Selection::Yes);
        assert_eq!(action, KeyAction::Cancel);
    }

    #[test]
    fn test_process_key_other_keys_continue() {
        let action = process_key(KeyCode::Char('x'), KeyModifiers::empty(), Selection::Yes);
        assert_eq!(action, KeyAction::Continue(Selection::Yes));

        let action = process_key(KeyCode::Up, KeyModifiers::empty(), Selection::No);
        assert_eq!(action, KeyAction::Continue(Selection::No));
    }
}
