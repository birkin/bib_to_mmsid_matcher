// #[macro_use]
extern crate log;
extern crate env_logger;

// use env_logger::Env;
// use std::env;
// use std::process;


// use log::{debug, info, error};
use env_logger::{Builder, Target};



#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
    logger_json_file_path: String,
    max_entries: i8  // this could be added to the json-file instead
}


impl Config {
    /*  forgive the "RUST_LOG" hack; i want to use the envar project-prefix to set the log-level,
        ...and couldn't figure out how to specify an alternative prefix for env_logger::init() */
    fn new() -> Config {
        match envy::prefixed("LOG_ROTATOR__").from_env::<Config>() {  // https://github.com/softprops/envy
            Ok(config) => {
                env::set_var( "RUST_LOG", &config.log_level);
                let log_level = config.log_level;  // not used, but still useful to set, for panic-message if it's missing
                let logger_json_file_path = config.logger_json_file_path;
                let max_entries = config.max_entries;
                Config { log_level, logger_json_file_path, max_entries }
            },
            Err(error) => panic!("{:#?}", error) // this shows the missing envar
        }
    }
}



fn main() {    
    env_logger::init();

    debug!( "this is a debug ``{}``", "message" );
    info!( "this is an info ``{}``", "message" );
    error!( "this is an error ``{}``", "message" );

    println!("Hello, world!");
}
