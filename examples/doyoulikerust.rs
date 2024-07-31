use yes_or_no::yes_or_no;

fn main() {
    if yes_or_no("🦀 Do you like Rust? 🦀", true) {
        println!("You like Rust! 🤩" );
    } else {
        println!("You don't like Rust... 😭");
    }
}
