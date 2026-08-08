use std::error::Error;
use tracing::{info};
use clap::{Parser, ValueEnum};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

mod stack;
mod ble;
mod heart_rate;
mod power;
mod mock;
mod sliding_window;
mod heap;
mod aggregator;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum HeartRateSource {
    Ble,
    Mock,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum PowerSource {
    Ble,
    Mock,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, value_enum)]
    hr_source: Option<HeartRateSource>,
    #[arg(long, value_enum)]
    power_source: Option<PowerSource>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting BikeMate9000...");

    let args = Cli::parse();

    // default to bluetooth
    let hr_source = match args.hr_source {
        None => HeartRateSource::Ble,
        Some(arg) => arg
    };

    let power_source = match args.power_source {
        None => PowerSource::Ble,
        Some(arg) => arg
    };

    let token = CancellationToken::new();
    let token_2 = token.clone();
    let token_3 = token.clone();
    let token_4 = token.clone();

    // create channel and spawn tasks
    let (tx, mut rx) = mpsc::channel::<aggregator::Event>(32);

    let t1 = tokio::spawn(aggregator::aggregate(rx, token_2));
    let t2 = tokio::spawn(heart_rate::ingest_heart_rate(hr_source, tx.clone(), token_3));
    let t3 = tokio::spawn(power::ingest_power(power_source, tx.clone(), token_4));
    

    // Listen for interrupt
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                token.cancel();
                break;
            }
        }
    }

    // Wait for threads to shutdown cleanly
    let _ = t1.await;
    let _ = t2.await;
    let _ = t3.await;

    Ok(())
}
