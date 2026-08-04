use std::error::Error;
use tracing::{info, warn, error, debug, trace};
mod stack;
mod ble;
mod heart_rate;
mod mock;
mod sliding_window;
use clap::{Parser, ValueEnum};

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

    let hr_source = match args.hr_source {
        None => HeartRateSource::Ble,
        Some(arg) => arg
    };

    let mut hr_stream = heart_rate::get_heart_rate_stream(hr_source).await;
    heart_rate::do_heart_rate_stuff(&mut hr_stream).await;

    Ok(())
}
