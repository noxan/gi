use std::{env, io};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    println!("The home directory is {}", home_dir.display());

    let current_dir = env::current_dir()?;
    println!("The current directory is {}", current_dir.display());

    Ok(())
}
