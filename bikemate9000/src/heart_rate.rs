use futures_util::{Stream, StreamExt};
use tracing::{info, warn, error, debug, trace};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ble;
use crate::mock;
use crate::HeartRateSource;
use crate::stack;

pub type HeartRateReading = u16;
pub type HeartRateStream = std::pin::Pin<Box<dyn Stream<Item = HeartRateReading> + Send>>;

#[derive(Clone, Copy, Debug)]
pub struct RichHeartRateReading {
    pub ts: u128,
    pub hr_reading: HeartRateReading
}

pub async fn get_heart_rate_stream(hr_source: HeartRateSource) -> HeartRateStream {
    match hr_source {
        HeartRateSource::Ble => { ble::get_heart_rate_stream().await },
        HeartRateSource::Mock => { mock::get_heart_rate_stream().await }
    }
}

pub async fn do_heart_rate_stuff(hr_stream: &mut HeartRateStream) {
    let mut stack = stack::MinStack::new();
    while let Some(hr_reading) = hr_stream.next().await {
        info!("hr reading: {:?}", hr_reading);

        // timestamp
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = since_the_epoch.as_millis();

        stack.push(RichHeartRateReading{ ts: ts, hr_reading: hr_reading });
        info!("stack: {:?}", stack)
    }
}


