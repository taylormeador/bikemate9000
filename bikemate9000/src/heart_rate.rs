use futures_util::{Stream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::{info};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Sender;

use crate::ble;
use crate::mock;
use crate::HeartRateSource;

pub type RawHeartRateReading = u16;
pub type HeartRateStream = std::pin::Pin<Box<dyn Stream<Item = RawHeartRateReading> + Send>>;

#[derive(Clone, Copy, Debug)]
pub struct HeartRateReading {
    pub ts: u128,
    pub hr_reading: RawHeartRateReading
}

pub async fn get_heart_rate_stream(hr_source: HeartRateSource) -> HeartRateStream {
    match hr_source {
        HeartRateSource::Ble => { ble::get_heart_rate_stream().await },
        HeartRateSource::Mock => { mock::get_heart_rate_stream().await }
    }
}

async fn handle_reading(hr_reading: RawHeartRateReading, hr_tx: &Sender<HeartRateReading>) {
    info!("Heart rate reading: {:?}", hr_reading);

    // Enrich with timestamp
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = since_the_epoch.as_millis();

    hr_tx.send(HeartRateReading{ ts: ts, hr_reading: hr_reading }).await.unwrap();
}

pub async fn ingest_heart_rate(hr_source: HeartRateSource, hr_tx: Sender<HeartRateReading>, token: CancellationToken) {
    let mut hr_stream = get_heart_rate_stream(hr_source).await;

    // Race between message received and ctrl-c
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                info!("TODO shutting down hr ingestion...");
                break;
            }
            msg = hr_stream.next() => { 
                match msg {
                    Some(msg) => { handle_reading(msg, &hr_tx).await }
                    None => break
                }
            }
        }
    }
}


