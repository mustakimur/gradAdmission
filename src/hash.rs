extern crate argon2rs;
extern crate rand;

use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io;
use std::iter;

fn main() -> io::Result<()> {
    println!(
        "This program helps you to create an initial user for the server.\n\
         It askes you for your password and creates a random salt for you.\n\
         It then calculates and shows the hash of the salted password.\n\
         You can use the DB Browser for SQLite to add the user to the database.\n\
         Set the role to Sys to create an admin user.\n",
    );

    println!("Type in your password:");

    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd)?;
    let passwd = passwd.trim_right();

    let mut rng = rand::thread_rng();

    let salt: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(16)
        .collect();

    let ba = argon2rs::argon2d_simple(&passwd, &salt);
    let strs: Vec<String> = ba.iter().map(|b| format!("{:02X}", b)).collect();
    let hash = strs.join("");

    println!(
        "Your password is:[{}], your salt is:[{}], \nyour hash is: [{}]\n\
         Do NOT copy the brackets. They are the deliminators.",
        passwd, salt, hash
    );

    Ok(())
}
