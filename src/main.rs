use chrono::{Local, TimeZone};
use chrono::{NaiveDate, Duration};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use std::fs;
use std::io;
use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = init_connection().unwrap();

    match ST::from_args() {
        ST::Start { record } => {
            println!("Start {}", record);
            connection.execute(
                "INSERT INTO TASKS (record, start) VALUES (?1, ?2)",
                &[record, get_timestamp()],
            )?;
        }
        ST::Stop {} => {
            println!("Stop");
            connection.execute(
                "UPDATE tasks SET end = ?1 WHERE end IS NULL",
                &[get_timestamp()],
            )?;
        }
        ST::Report { rdate } => {
            let rd = if rdate == "today" {
                Local::today().naive_utc().and_hms(0, 0, 0)
            } else {
                NaiveDate::parse_from_str(&rdate, "%Y-%m-%d")
                    .unwrap()
                    .and_hms(0, 0, 0)
            };
            let date = Local.from_local_datetime(&rd);
            let ts = date.unwrap().timestamp();
            let ts24 = ts + 24 * 60 * 60;
            let mut stmt = connection.prepare(
                "SELECT record, SUM(end - start) FROM tasks WHERE start >= ?1 AND start <= ?2 AND end IS NOT NULL GROUP BY record",
            )?;
            let tasks = stmt.query_map(&[ts, ts24], |row| {
                Ok(Task {
                    record: row.get(0)?,
                    start: 0,
                    end: 0,
                    duration: row.get(1)?,
                })
            })?;
            let mut total: u32 = 0;
            for t in tasks {
                let tt = t.unwrap();
                let duration = Duration::seconds(tt.duration as i64);
                println!("Task: {}. Duration: {}h {}m {}s", tt.record, duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60);
                total = total + tt.duration;
            }
            let total_duration = Duration::seconds(total as i64);
            println!("Total this day: {}h {}m {}s", total_duration.num_hours(), total_duration.num_minutes() % 60, total_duration.num_seconds() % 60);
        }
        ST::Status {} => {
            let mut stmt =
                connection.prepare("SELECT record, start FROM tasks WHERE end IS NULL")?;
            let tasks = stmt.query_map(NO_PARAMS, |row| {
                Ok(Task {
                    record: row.get(0)?,
                    start: row.get(1)?,
                    end: 0,
                    duration: 0,
                })
            })?;
            for t in tasks {
                println!("Current task: {}", t.unwrap().record);
            }
        }
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(about = "simple timer")]
enum ST {
    #[structopt(alias="st")]
    Start {
        record: String,
    },
    #[structopt(alias="sp")]
    Stop,
    #[structopt(alias="ss")]
    Status,
    #[structopt(alias="r")]
    Report {
        #[structopt(default_value = "today", short="d")]
        rdate: String,
    },
}

struct Task {
    record: String,
    start: u32,
    end: u32,
    duration: u32,
}

fn init_connection() -> Result<Connection> {
    let path = init_path().unwrap();
    let conn = Connection::open(&path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
             id INTEGER PRIMARY KEY,
             record TEXT NOT NULL,
             start INTEGER NOT NULL,
             end INTEGER
         )",
        NO_PARAMS,
    )?;
    Ok(conn)
}

fn init_path() -> io::Result<PathBuf> {
    let mut path = dirs_next::home_dir().unwrap();
    path.push(".stimer");
    if !path.exists() {
        fs::create_dir(path.clone())?;
    }
    path.push("stimer.db");
    Ok(path)
}

fn get_timestamp() -> String {
    let n = SystemTime::now();
    let t = n.duration_since(UNIX_EPOCH).expect("Time went backwards");
    t.as_secs().to_string()
}
