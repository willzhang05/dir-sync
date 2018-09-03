extern crate dns_lookup;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use dns_lookup::{lookup_host, lookup_addr};

#[derive(Serialize, Deserialize, Debug)]

struct Directory {
    uri: String
}

fn main() {
    println!("Hello, world!");
}
