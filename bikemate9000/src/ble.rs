use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn, error, debug, trace};
use futures_util::StreamExt;
use uuid::Uuid;
use crate::heart_rate::{HeartRateStream, HeartRateReading};

const HEART_RATE_MONITOR_NAME: &str = "HRM-Dual:863788";
const HEART_RATE_MEASUREMENT: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

async fn connect_to_garmin() -> Result<Peripheral, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        info!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            error!("BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                if local_name == HEART_RATE_MONITOR_NAME {
                    info!("Found Garmin heart rate strap!");
                } else {
                    continue;
                }
                if !is_connected {
                    info!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        error!("Error connecting to Peripheral: {}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                info!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected, &local_name
                );
                return Ok(peripheral.clone());
            }
        }
    }
    Err(Box::from("Error connecting to Garmin heart rate monitor"))
}

pub async fn get_heart_rate_stream() -> HeartRateStream {
    let peripheral = connect_to_garmin().await.expect("garmin error handling not implemented");
    peripheral.discover_services().await.expect("failed to discover services");
    let chars = peripheral.characteristics();
    let hr_char = chars.iter().find(|c| c.uuid == HEART_RATE_MEASUREMENT).expect("Unable to find heart rate measurement characteristic");
    peripheral.subscribe(hr_char).await.expect("failed to subscribe");

    let raw_stream = peripheral.notifications().await.expect("failed to get notification stream");
    let parsed_stream = raw_stream.map(|raw_notification| parse_heart_rate(&raw_notification.value));
    let hr_stream: HeartRateStream = Box::pin(parsed_stream);

    hr_stream
}

fn parse_heart_rate(value: &[u8]) -> HeartRateReading {
    let flags = value[0];
    let hr_16bit = flags & 0x01 != 0;

    if hr_16bit {
        u16::from_le_bytes([value[1], value[2]])
    } else {
        value[1] as u16
    }
}