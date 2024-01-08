use std::{env, io};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let current_dir = env::current_dir()?;
    println!("The current directory is {}", current_dir.display());

    Ok(())
}
