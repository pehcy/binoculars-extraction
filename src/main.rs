mod models;

extern crate lazy_static;
extern crate postgres;

use models::OHLCVData;
use postgres::{Client, NoTls};

use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::error::Error;
use std::env;
use dotenv::dotenv;

macro_rules! createdb_fmt_str {
    () => {
        "
        CREATE TABLE IF NOT EXISTS {} (
            id                  SERIAL PRIMARY KEY,
            open_time           BIGINT,
            open                REAL,
            high                REAL,
            low                 REAL,
            close               REAL,
            volume              REAL,
            closetime           BIGINT,
            quote_asset_volume  REAL,
            num_of_trades       INTEGER,
            taker_by_quote      REAL,
            taker_buy_quote     REAL,
            ignore              INTEGER
        );
        "
    };
}

macro_rules! insert_fmt_str {
    () => {
        "INSERT INTO {} (
            open_time,
            open,high,
            low,
            close,
            volume,
            closetime,
            quote_asset_volume,
            num_of_trades,
            taker_by_quote,
            taker_buy_quote,
            ignore) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scheduling_task = tokio::spawn(async {
        // wait for 15 minutes
        let mut loop_interval = tokio::time::interval(Duration::from_secs(60 * 15));

        loop {
            loop_interval.tick().await;
            println!("starting another period...");

            let mut queue = Vec::<JoinHandle<()>>::new();

            let symbols: Vec<&str> = vec!["ADAUSDT", "ADABNB"];

            let inv: String = "15m".to_string();
            
            for symb in symbols {
                let res = OHLCVData::get(&symb.to_string(), &inv).await.unwrap();
                let handle = thread::spawn(move || {
                    for i in res {
                        println!("data here: {:?}", i);
                        prepare_database(i, &symb.to_string()).ok();
                    }
                });
                queue.push(handle);
            }

            // wait for each thread to finish and insert ohlcv into database
            for handle in queue {
                handle.join().unwrap();
            }
        }
    });

    scheduling_task.await;
    
    Ok(())
}

fn prepare_database(data: OHLCVData, symbol: &str) -> Result<(), postgres::Error> {
    // load environment variables
    dotenv().ok();
    let posgres_addr = env::var("DATABASE_URL").expect("ERROR: Database URL is undefined.");
    println!("database address: {:?}", posgres_addr);

    let mut client = Client::connect(&posgres_addr.to_string(), NoTls)?;

    // create table
    client.batch_execute(&format!(createdb_fmt_str!(), &symbol));

    client.execute(
        &format!(insert_fmt_str!(), &symbol),
        &[
            &(data.opentime as i64),
            &data.open,
            &data.high,
            &data.low,
            &data.close,
            &data.volume,
            &(data.closetime as i64),
            &data.quote_asset_volume,
            &(data.num_of_trades as i32),
            &data.taker_by_quote,
            &data.taker_buy_quote,
            &data.ignore
        ]
    )?;

    println!("Inserted into database");

    Ok(())
}
