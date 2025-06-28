#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() {
    let client = HttpClient::new();
    match client.get("example.com".to_string(), 80, "/".to_string()) {
        Ok(response) => {
            print!("response:\n{:#?}", response);
        }
        Err(e) => {
            println!("Error: {:#?}", e);
        }
    }
}

entry_point!(main);
