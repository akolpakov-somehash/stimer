use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    match ST::from_args() {
        ST::Start {record} => {
            println!("Start {}", record)
        }
        ST::Stop {} => {
            println!("Stop")
        }
        ST::Report {start, end} => {
            println!("Start {}, end {}", start, end)
        }
    }
    Ok(())
}


#[derive(StructOpt)]
#[structopt(about = "simple timer")]
enum ST {
    Start {
        record: String
    },
    Stop {

    },
    Report {
        #[structopt(short)]
        start: String,
        #[structopt(short)]
        end: String
    }
}

