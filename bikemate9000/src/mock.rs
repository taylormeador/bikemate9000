use tokio::time::{Duration, interval};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use rand::random_range;

use crate::heart_rate::{HeartRateStream, HeartRateReading};

pub async fn get_heart_rate_stream() -> HeartRateStream {
    let interval = interval(Duration::from_secs(1));
    Box::pin(IntervalStream::new(interval).map(|_| generate_reading()))
}

fn generate_reading() -> HeartRateReading {
    random_range(50..200) as u16
}
