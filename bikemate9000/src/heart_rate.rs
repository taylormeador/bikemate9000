use futures_util::{Stream, StreamExt};
use tracing::{info, warn, error, debug, trace};

use crate::ble;
use crate::mock;
use crate::HeartRateSource;

pub type HeartRateReading = u16;
pub type HeartRateStream = std::pin::Pin<Box<dyn Stream<Item = HeartRateReading> + Send>>;

pub async fn get_heart_rate_stream(hr_source: HeartRateSource) -> HeartRateStream {
    match hr_source {
        HeartRateSource::Ble => { ble::get_heart_rate_stream().await },
        HeartRateSource::Mock => { mock::get_heart_rate_stream().await }
    }
}

pub async fn do_heart_rate_stuff(hr_stream: &mut HeartRateStream) {
    while let Some(hr_reading) = hr_stream.next().await {
        info!("hr reading: {:?}", hr_reading);
    }
}