// NOCOM(#sirver): what
#![allow(unused_imports)]

use anyhow::{bail, Result};
use argh::FromArgs;
use btleplug::api::{Peripheral, WriteType};
use futures::prelude::*;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;
use tokio::{task, time};

#[derive(FromArgs)]
/// Communicates to the BT Smart Controller via Bluetooth and proxies reads and writes to MQTT.
struct Args {
    /// mqtt host to connect to
    #[argh(option, default = "\"localhost\".to_string()")]
    host: String,

    /// mqtt port to connect to
    #[argh(option, default = "1883")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let mut mqttoptions = MqttOptions::new("ftbtc", &args.host, args.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("ftbtc/m1", QoS::AtMostOnce).await?;
    client.subscribe("ftbtc/m2", QoS::AtMostOnce).await?;

    let device = ft_ble::open_device().await?;

    let mut notification_stream = device.peripheral.notifications().await?;
    loop {
        tokio::select! {
            val = notification_stream.next() => {
                let Some(val) = val else { continue; };
                let (topic, value) = match &val.uuid.to_string() as &str {
                    ft_ble::UUID_CHAR_BATTERY => ("battery", val.value[0] as u16),
                    ft_ble::UUID_CHAR_I1 => ("i1", u16::from_le_bytes(val.value.try_into().unwrap())),
                    ft_ble::UUID_CHAR_I2 => ("i2", u16::from_le_bytes(val.value.try_into().unwrap())),
                    ft_ble::UUID_CHAR_I3 => ("i3", u16::from_le_bytes(val.value.try_into().unwrap())),
                    ft_ble::UUID_CHAR_I4 => ("i4", u16::from_le_bytes(val.value.try_into().unwrap())),
                    _ => {
                        println!("Unknown channel: {val:?}");
                        continue;
                    }
                };
                client.publish(&format!("ftbtc/{topic}"), QoS::AtLeastOnce, false, format!("{value}").as_bytes()).await?;
            }
            e = eventloop.poll() => {
                let Event::Incoming(Packet::Publish(p)) = e? else {
                    continue;
                };
                // Signed byte, ccw is negative, cw is positive.
                let v: i8 = serde_json::from_slice(&p.payload)?;
                let c = match &p.topic as &str {
                    "ftbtc/m1" => &device.char_m1,
                    "ftbtc/m2" => &device.char_m2,
                    _ => unreachable!("Not subscribed to anything else."),
                };
                device.peripheral.write(c, &[v as u8], WriteType::WithResponse).await?;
            }
        }
    }
}
