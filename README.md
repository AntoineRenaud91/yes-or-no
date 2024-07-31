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

fn main() {
    if yes_or_no("🦀 Do you like Rust? 🦀", true) {
        println!("You like Rust! 🤩" );
    } else {
        println!("You don't like Rust... 😭");
    }
}
```
When you run the program, you'll be presented with a prompt in your terminal:
```bash
Do you like Rust? Yes [✓] No [ ]
```
- Navigate between "Yes" and "No" using the left and right arrow keys.
- Press Enter to select your choice.
- Press Escape to automatically select "No".

Depending on your selection, the program will output:
```bash
You like Rust! 😁
```
or
```bash
You don't like Rust... 😭
```
