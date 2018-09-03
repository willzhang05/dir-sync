extern crate dns_lookup;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;

use dns_lookup::{lookup_host, lookup_addr};

use serde_json::{Value, Error};


struct Directory {
    uri: String
}

fn read_in() -> Result<(), Error> {
    let mut config_file = File::open("config.json").unwrap();
    let mut data = String::new();
    config_file.read_to_string(&mut data).unwrap();
    
    let mut v: Value = serde_json::from_str(&data)?;
    println!("{} {}", v["servers"]["name"], v["servers"]["address"]);

    Ok(())
}

fn main() {
    
    /*for argument in env::args_os() {
        println!("{:?}", argument);    
    }*/

    let result = read_in();
    let result = match result {
         Ok(()) => (),
        Err(error) => {
            panic!("There was a problem reading in config.json: {:?}", error)
        },
    };
    
}
