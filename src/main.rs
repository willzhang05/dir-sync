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

fn read_in(servers: &mut Vec<Value>, local_dirs: &mut Vec<Value>) -> Result<(), Error> {
    let mut config_file = File::open("config.json").unwrap();
    let mut data = String::new();
    config_file.read_to_string(&mut data).unwrap();
    
    let v: Value = serde_json::from_str(&data)?;

    //println!("{:?} {:?}", v["servers"][0]["name"], v["servers"][0]["address"]);

    let server_vec: Vec<Value> = v["servers"].as_array().unwrap().to_vec();
    
    for s in server_vec {
        println!("{:?} {:?}", s["name"], s["hostname"]);
        servers.push(s);
    }
 
    let ld_vec: Vec<Value> = v["local_dir"].as_array().unwrap().to_vec();

    for dir in ld_vec {
        println!("{:?} {:?}", dir["label"], dir["uri"]);
        local_dirs.push(dir);
    }

    Ok(())
}

/*
fn check_dir(local_dirs: &Vec<Value>) {
    
}
*/

fn check_host(hostname: String) -> bool {
    let lookup_result = lookup_host(&hostname);
    let lookup_result = match lookup_result {
        Ok(()) => true,
        Err(error) => {
            panic!("Invalid hostname specified in config: {:?}", error)
        },
    };
    lookup_result

}

fn main() {
    
    /*for argument in env::args_os() {
        println!("{:?}", argument);    
    }*/
    let mut servers: Vec<Value> = vec![];
    let mut local_dirs: Vec<Value> = vec![];

    let result = read_in(&mut servers, &mut local_dirs);
    let result = match result {
        Ok(()) => (),
        Err(error) => {
            panic!("There was a problem reading in config.json: {:?}", error)
        },
    };
    
    for server in servers {
        check_host(server["hostname"].to_string());
    }
    

}
