use std::error::Error;
use tracing::{info, warn, error, debug, trace};
mod stack;
mod ble;
mod heart_rate;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum HeartRateSource {
    Ble,
    Mock,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_enum)]
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

    let hr_stream = heart_rate::get_heart_rate_stream(hr_source);
    heart_rate::do_heart_rate_stuff(hr_stream);

    Ok(())
}
