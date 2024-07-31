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
//
//! ## Usage:
//! Here is an example of how to use `yes-or-no` in your Rust project:
//! ```rust
//! use yes_or_no::yes_or_no;
//!
//! fn main() {
//!     if yes_or_no("🦀 Do you like Rust? 🦀", true) {
//!         println!("You like Rust! 🤩" );
//!     } else {
//!        println!("You don't like Rust... 😭");
//!     }
//! }
//! ```
//! When you run the program, you'll be presented with a prompt in your terminal:
//! ```bash
//! Do you like Rust? Yes [✓] No [ ]
//! ```
//! - Navigate between "Yes" and "No" using the left and right arrow keys.
//! - Press Enter to select your choice.
//! - Press Escape to automatically select "No".
//!
//! Depending on your selection, the program will output:
//! ```bash
//! You like Rust! 😁
//! ```
//! or
//! ```bash
//! You don't like Rust... 😭
//! ```



use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyModifiers}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::io::{self, Write};

pub fn yes_or_no(question:&str, is_yes_initial: bool) -> bool {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let mut yes = is_yes_initial;

    loop {
        execute!(stdout,cursor::MoveTo(0, 0),crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)).unwrap();
        if yes {
            print!("{question} Yes [✓] No [ ]")
        } else {
            print!("{question} Yes [ ] No [✓]")
        }
        io::stdout().flush().unwrap();
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => {
                    yes = true;
                }
                KeyCode::Right => {
                    yes = false;
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Esc => {
                    yes = false;
                    break;
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout,LeaveAlternateScreen).unwrap();    
                    disable_raw_mode().unwrap();
                    println!("Manually interupted.");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
    execute!(stdout,LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
    yes
}