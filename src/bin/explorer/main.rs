// NOCOM(#sirver): what
#![allow(unused_imports)]

use anyhow::Result;
use btleplug::api::Peripheral;
use btleplug::api::{Central, CharPropFlags, Manager as _, ScanFilter, WriteType};
use btleplug::platform::Manager;
use ft_ble::open_device;
use futures::prelude::*;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let device = open_device().await?.peripheral;

    device.discover_services().await?;
    let characteristics = device.characteristics();

    let mut read_chars = Vec::new();
    for characteristic in characteristics.iter() {
        println!("#sirver characteristic: {:#?}", characteristic);
        if characteristic.properties.contains(CharPropFlags::NOTIFY) {
            // Subscribe to notifications
            device.subscribe(characteristic).await?;

            // while let Some(data) = notification_stream.next().await {
            // println!(
            // "Received data from characteristic {:?}: {:?}",
            // data.uuid, data.value
            // );
            // }
        }

        if characteristic.properties.contains(CharPropFlags::READ) {
            read_chars.push(characteristic);
        }

        if characteristic.properties.contains(CharPropFlags::WRITE) {
            if characteristic.uuid.to_string() == "8ae88b84-ad7d-11e6-80f5-76304dec7eb7" {
                device
                    .write(&characteristic, &[0x81], WriteType::WithResponse)
                    .await?;
            }
        }
    }

    // loop {
    println!("Loop start");
    for c in &read_chars {
        let v = device.read(c).await?;
        println!("#sirver c: {}, v: {:?}", c.uuid, v);
    }
    // }

    let mut notification_stream = device.notifications().await?;
    while let Some(data) = notification_stream.next().await {
        println!(
            "Received data from characteristic {:?}: {:?}",
            data.uuid, data.value
        );
    }

    Ok(())
}
