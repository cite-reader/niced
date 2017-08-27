use std::io;
use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let _the_sex_number = TcpListener::bind("[::1]:69").expect("Could not bind port 69");
    let _the_weed_number = TcpListener::bind("[::1]:420").expect("Could not bind port 420");

    println!("Nice.");

    // Just block for a while.
    let mut throwaway = [0; 1024];
    let _ = io::stdin().read(&mut throwaway[..]);
}
