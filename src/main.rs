extern crate dns_lookup;

extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

//use std::env;
use std::fs::File;
use std::fs::DirEntry;
use std::io::Read;

use dns_lookup::{lookup_host, lookup_addr};

use serde_json::{Value, Error};


struct Directory {
    uri: String
}

fn read_in<'servers,'local_dirs>(servers: &mut Vec<Value>, local_dirs: &mut Vec<Value>) -> Result<(&'servers Vec<Value>, &'local_dirs Vec<Value>), Error> {
    let mut config_file = File::open("config.json").unwrap();
    let mut data = String::new();
    config_file.read_to_string(&mut data).unwrap();
    
    let v: Value = serde_json::from_str(&data)?;

    //println!("{:?} {:?}", v["servers"][0]["name"], v["servers"][0]["address"]);

    let server_vec: Vec<Value> = v["servers"].as_array().unwrap().to_vec();
    servers = Vec::new();
    
    for s in server_vec {
        println!("{:?} {:?}", s["name"], s["hostname"]);
    }
 
    let ld_vec: Vec<Value> = v["local_dir"].as_array().unwrap().to_vec();
    local_dirs = Vec::new();

    for dir in ld_vec {
        println!("{:?} {:?}", dir["label"], dir["uri"]);
    }

    Ok((servers, local_dirs))
}

fn main() {
    
    /*for argument in env::args_os() {
        println!("{:?}", argument);    
    }*/
    let servers: Vec<Value>;
    let local_dirs: Vec<Value>;

    let result = read_in(&mut servers, &mut local_dirs);
    let result = match result {
        Ok((s, l)) => (s, l),
        Err(error) => {
            panic!("There was a problem reading in config.json: {:?}", error)
        },
    };

}
