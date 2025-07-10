#![allow(unused_imports)]
use r_cli::Config;
use r_cli::run;
use std::{env, process};


fn main(){
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    if let Err(e) = r_cli::run(config){
        println!("Application error {e}");
        process::exit(1);
    }
}
