#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
}

impl Config {
    /*  forgive the "RUST_LOG" hack; i want to use the envar project-prefix to set the log-level,
    ...and couldn't figure out how to specify an alternative prefix for env_logger::init() */
    fn new() -> Config {
        match envy::prefixed("MARC_BUILDER__").from_env::<Config>() {
            // https://github.com/softprops/envy
            Ok(config) => {
                env::set_var("RUST_LOG", &config.log_level);
                let log_level = config.log_level; // not used, but still useful to set, for panic-message if it's missing
                Config { log_level }
            }
            Err(error) => panic!("{:#?}", error), // this shows the missing envar
        }
    }
}

#[tokio::main]
async fn main() {
    /* configure settings */
    let config = Config::new();
    // println!("config, ``{:?}``", config);

    /* setup logging */
    let mut log_builder = Builder::from_default_env();
    log_builder.target(Target::Stdout);
    log_builder.init();
    debug!("settings, ``{:?}``", config);

    /* initialize sqlite file */

    /* get list of marc files */

    /*  for each file... */

    /*  open file */

    /* for each marc-reacord... iterate through marc-record-data */

    /* pull out old-bib */

    /* pull out mms_id */

    /* save to sqlite3 file */

    debug!("{}", format!("config, ``{:#?}``", config)); // debug! needs a string literal

    debug!("this is a debug: ``{}``", "message");
    info!("this is an info: ``{}``", "message");
    error!("this is an error: ``{}``", "message");

    println!("Hello, world!");
}
