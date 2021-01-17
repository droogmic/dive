use std::io::{self, Read};

use dive::{Secret, User, UserVerifier};

fn user_secret() {
    println!("Hello, world!");
    let user = User::new_dalek();
    println!("Hello user {}", user.id);
    println!("Public key: {:x?}", user.get_public_key().as_bytes());
    println!();
    println!("What is your secret? ");
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let secret = Secret { data: buffer };
    let signature = user.sign(&secret);
    println!("Secret's signature: {:#?}", signature);
    println!();

    let user_verifier = UserVerifier {
        verify_key: user.get_public_key(),
    };
    loop {
        println!("What was your secret? ");
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let secret = Secret { data: buffer };
        match user_verifier.verify(&secret, &signature) {
            Ok(_) => println!("Yes it was!"),
            Err(_) => println!("I don't believe you"),
        }
    }
}

fn main() {
    user_secret();
}
