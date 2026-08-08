use tracing::{info};
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;

use crate::heart_rate::HeartRateReading;
use crate::power::PowerReading;

pub enum Event {
    HeartRate(HeartRateReading),
    Power(PowerReading)
}

async fn handle_hr_reading(msg: HeartRateReading) {
    info!("HR reading: {:?}", msg.hr);
}

async fn handle_power_reading(msg: PowerReading) {
    info!("Power reading: {:?}", msg.power)
}

pub async fn aggregate(mut rx: mpsc::Receiver<Event>, token: CancellationToken) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                info!("TODO shutting down aggregator...");
                break;
            }
            msg = rx.recv() => { 
                match msg {
                    Some(Event::HeartRate(m)) => { handle_hr_reading(m).await },
                    Some(Event::Power(m)) => {handle_power_reading(m).await },
                    None => break
                }
            }
        }
    }
}
