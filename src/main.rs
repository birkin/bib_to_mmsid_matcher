// #[macro_use]
extern crate log;
extern crate env_logger;

// use env_logger::Env;
// use std::env;
// use std::process;


use log::{debug, info, error};






fn main() {    
    env_logger::init();

    debug!( "this is a debug ``{}``", "message" );
    info!( "this is an info ``{}``", "message" );
    error!( "this is an error ``{}``", "message" );

    println!("Hello, world!");
}
