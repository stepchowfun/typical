#![deny(warnings)]

mod types;

use {
    std::{f64::consts::PI, process::exit},
    types::{Deserialize, Serialize},
};

fn main() {
    let mut buffer = Vec::<u8>::new();

    if let Err(err) = PI.serialize(&mut buffer) {
        eprintln!("{}", err);
        exit(1);
    }

    println!("Pi as bytes: {:?}", buffer);

    let pi = match f64::deserialize(&mut buffer.as_slice()) {
        Ok(pi) => pi,
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1);
        }
    };

    println!("Pi from those bytes: {}", pi);

    assert!(pi == PI);
}
