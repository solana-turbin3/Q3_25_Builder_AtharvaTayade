use bs58 ;
use std::io::{self, BufRead};

//Function to convert a base58 encoded string to a byte vector
#[test]
fn base58_to_wallet(){
    println!("Enter your private key in base58 format:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file format is: ");

    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}