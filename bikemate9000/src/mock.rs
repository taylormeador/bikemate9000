use tokio::time::{Duration, interval};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use rand::random_range;

use crate::heart_rate::HeartRateStream;
use crate::power::PowerStream;

pub async fn get_heart_rate_stream() -> HeartRateStream {
    let interval = interval(Duration::from_secs(1));
    Box::pin(IntervalStream::new(interval).map(|_| generate_reading()))
}

pub async fn get_power_stream() -> PowerStream {
    let interval = interval(Duration::from_secs(1));
    Box::pin(IntervalStream::new(interval).map(|_| generate_reading()))
}

fn generate_reading() -> u16 {
    random_range(50..200) as u16
}
