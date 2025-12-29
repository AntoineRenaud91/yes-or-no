# yes-or-no
A very simple yes or no CLI tool

## Installation

Add `yes-or-no` to your project's dependencies:
```toml
[dependencies]
yes-or-no = "0.1"
```

## Usage:
Here is an example of how to use `yes-or-no` in your Rust project:
```rust
use yes_or_no::yes_or_no;

fn main() -> std::io::Result<()> {
    match yes_or_no("🦀 Do you like Rust? 🦀", true)? {
        Some(true) => println!("You like Rust! 🤩"),
        Some(false) => println!("You don't like Rust... 😭"),
        None => println!("Cancelled."),
    }
    Ok(())
}
```
When you run the program, you'll be presented with a prompt in your terminal:
```bash
🦀 Do you like Rust? 🦀 Yes [✓] No [ ]
```
- Navigate between "Yes" and "No" using the left and right arrow keys.
- Press Enter to select your choice.
- Press Escape to select "No".
- Press Ctrl+C to cancel (returns `None`).

## Return Values

The function returns `io::Result<Option<bool>>`:
- `Ok(Some(true))` - User selected "Yes"
- `Ok(Some(false))` - User selected "No" (or pressed Escape)
- `Ok(None)` - User cancelled with Ctrl+C
- `Err(_)` - A terminal I/O error occurred
