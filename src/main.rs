use chrono::{Local, TimeZone};
use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = init_connection().unwrap();

    match ST::from_args() {
        ST::Start { record } => {
            println!("Start {}", record);
            connection.execute(
                "iNSERT INTO TASKS (record, start) VALUES (?1, ?2)",
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
                "SELECT record, start, end FROM tasks WHERE start >= ?1 AND start <= ?2",
            )?;
            let tasks = stmt.query_map(&[ts, ts24], |row| {
                Ok(Task {
                    record: row.get(0)?,
                    start: row.get(1)?,
                    end: row.get(2)?,
                    duration: 0,
                })
            })?;
            for t in tasks {
                println!("Tasks: {}", t.unwrap().record);
            }
        }
        ST::Status {} => {
            let mut stmt =
                connection.prepare("SELECT record, start, end FROM tasks WHERE end IS NULL")?;
            let tasks = stmt.query_map(NO_PARAMS, |row| {
                Ok(Task {
                    record: row.get(0)?,
                    start: row.get(1)?,
                    end: row.get(2)?,
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
    Start {
        record: String,
    },
    Stop,
    Status,
    Report {
        #[structopt(default_value = "today", short)]
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
    let conn = Connection::open("stimer.db")?;
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

fn get_timestamp() -> String {
    let n = SystemTime::now();
    let t = n.duration_since(UNIX_EPOCH).expect("Time went backwards");
    t.as_secs().to_string()
}
