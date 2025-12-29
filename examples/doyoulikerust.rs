use yes_or_no::yes_or_no;

fn main() -> std::io::Result<()> {
    match yes_or_no("🦀 Do you like Rust? 🦀", true)? {
        Some(true) => println!("You like Rust! 🤩"),
        Some(false) => println!("You don't like Rust... 😭"),
        None => println!("Cancelled."),
    }
    Ok(())
}
