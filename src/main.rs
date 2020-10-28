

#![allow(unused_imports)]

extern crate serde;
extern crate toml;
extern crate postgres;


use std::process::exit;
use std::io;

mod configuration;
mod postgresql;
mod model;
mod application;


fn main() {

    let mut sinput = String::new();
    match io::stdin().read_line(&mut sinput) {
        Ok( _ ) => {
            if sinput.is_empty() {
                println!("Goodbuy!");
                exit(0);
            }
        },
        Err( _error ) => {
            println!("Goodbuy!");
            exit(0);
        },
    };
    // println!("sinput :: {}", sinput);
    find_comments( &sinput.trim() );

    // find_comments( "0001.0003.0002.0002" );
    // find_comments( "0001" );
    
}

fn find_comments( base: &str ) {
    match application::App::run( base ) {
        Ok( _ ) => {
            // println!("Goodbuy!");
        },
        Err( error ) => {
            println!("Catch error :: {:?}", error);
        },
    }
}

