use futures_util::{Stream, StreamExt};
use tracing::{info};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ble;
use crate::mock;
use crate::HeartRateSource;
use crate::sliding_window;

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

pub async fn do_heart_rate_stuff(hr_stream: &mut HeartRateStream) {
    let mut five_sec_window = sliding_window::SlidingWindow::new(5000);
    while let Some(hr_reading) = hr_stream.next().await {
        info!("hr reading: {:?}", hr_reading);

        // Enrich with timestamp
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = since_the_epoch.as_millis();

        five_sec_window.handle_reading(HeartRateReading{ ts: ts, hr_reading: hr_reading });
        info!("min: {:?} max: {:?}", five_sec_window.get_min(), five_sec_window.get_max());
    }
}


