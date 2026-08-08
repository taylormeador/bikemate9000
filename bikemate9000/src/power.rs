use futures_util::{Stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::{info};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Sender;

use crate::ble;
use crate::mock;
use crate::PowerSource;
use crate::aggregator;

pub type RawPowerReading = u16;
pub type PowerStream = std::pin::Pin<Box<dyn Stream<Item = RawPowerReading> + Send>>;

#[derive(Clone, Copy, Debug)]
pub struct PowerReading {
    pub ts: u128,
    pub power: RawPowerReading
}

async fn handle_reading(power: RawPowerReading, tx: Sender<aggregator::Event>) {
    info!("Power reading: {:?}", power);

    // Enrich with timestamp
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = since_the_epoch.as_millis();

    let event = aggregator::Event::Power(PowerReading{ ts: ts, power: power });
    tx.send(event).await.unwrap();
}

pub async fn ingest_power(power_source: PowerSource, tx: Sender<aggregator::Event>, token: CancellationToken) {
    // TODO connect to power meter and figure out stream init
    // let mut power_stream = match power_source {
    //     PowerSource::Ble => { ble::get_power_stream().await },
    //     PowerSource::Mock => { mock::get_power_stream().await }
    // };
    info!("TODO implement power source instead of deaulting to {:?}", power_source);
    let mut power_stream = mock::get_power_stream().await;

    // Race between message received and ctrl-c
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                info!("TODO shutting down power ingestion...");
                break;
            }
            msg = power_stream.next() => { 
                match msg {
                    Some(msg) => { handle_reading(msg, tx.clone()).await }
                    None => break
                }
            }
        }
    }
}


