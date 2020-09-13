use std::env;
use std::process;
use std::error::Error;
use structopt::StructOpt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with parsing arguments: {}", err);
        process::exit(1);
    }
    );
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

#[derive(StructOpt)]
struct Cli {
    cmd: String,
    record: String,
}

struct Config {
    cmd: String,
    record: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() >3 {
            return Err("too much arguments");
        }
        let cmd = args[1].clone();
        let record = args[2].clone();

        Ok(Config {cmd, record})    
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("CMD {}", config.cmd);
    println!("record {}", config.record);

    Ok(())
}