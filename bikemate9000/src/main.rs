use std::error::Error;
use tracing::{info};
mod stack;
mod ble;
mod heart_rate;
mod mock;
mod sliding_window;
mod heap;
use clap::{Parser, ValueEnum};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum HeartRateSource {
    Ble,
    Mock,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, value_enum)]
    hr_source: Option<HeartRateSource>,
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

    let token = CancellationToken::new();
    let token_2 = token.clone();

    // create channels and spawn tasks
    let (hr_tx, mut hr_rx) = mpsc::channel::<heart_rate::HeartRateReading>(32);
    let t1 = tokio::spawn(heart_rate::ingest_heart_rate(hr_source, hr_tx, token_2));

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

    Ok(())
}
