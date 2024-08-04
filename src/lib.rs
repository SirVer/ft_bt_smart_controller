use anyhow::{bail, Result};
use btleplug::api::Peripheral;
use btleplug::api::{Central, CharPropFlags, Characteristic, Manager as _, ScanFilter, WriteType};
use btleplug::platform::Manager;
use std::error::Error;
use tokio::time::{sleep, Duration};

/// Seems to always send a single byte, maybe battery indicator in %
pub const UUID_CHAR_BATTERY: &str = "00002a19-0000-1000-8000-00805f9b34fb";
/// No idea, always sends [0, 0, 0, 0, 0, 248, 69, 16] for me.
pub const UUID_CHAR_UNKNOWN0: &str = "00002a23-0000-1000-8000-00805f9b34fb";
/// No idea, always sends [49, 54, 49, 57, 52, 52]
pub const UUID_CHAR_UNKNOWN1: &str = "00002a24-0000-1000-8000-00805f9b34fb";
/// No idea, always sends [49, 46, 54, 51, 0, 0, 0, 0]
pub const UUID_CHAR_UNKNOWN2: &str = "00002a26-0000-1000-8000-00805f9b34fb";
/// No idea, always sends [49, 0]
pub const UUID_CHAR_UNKNOWN3: &str = "00002a27-0000-1000-8000-00805f9b34fb";
/// When read always sends 'fischertechnik'
pub const UUID_CHAR_COMPANY: &str = "00002a29-0000-1000-8000-00805f9b34fb";
/// "Handle 0x001D", always sends [0] when read
pub const UUID_CHAR_UNKNOWN4: &str = "8ae87e32-ad7d-11e6-80f5-76304dec7eb7";
/// No idea, always sends [0] when read.
pub const UUID_CHAR_UNKNOWN5: &str = "8ae88224-ad7d-11e6-80f5-76304dec7eb7";
/// "Handle 0x0024", This is M1, sends 0 when LED is off
pub const UUID_CHAR_M1: &str = "8ae8860c-ad7d-11e6-80f5-76304dec7eb7";
/// "Handle 0x0026", This is M2, sends 0x81 when LED on
pub const UUID_CHAR_M2: &str = "8ae88b84-ad7d-11e6-80f5-76304dec7eb7";
/// These are for the input modes. If set to 0xb, this input returns OHM readings, if set to 0xa
/// it returns voltage readings.
pub const UUID_CHAR_MODE_I1: &str = "8ae88efe-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_MODE_I2: &str = "8ae89084-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_MODE_I3: &str = "8ae89200-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_MODE_I4: &str = "8ae89386-ad7d-11e6-80f5-76304dec7eb7";

/// Input channels, always 2 bytes when read. 0xffff means open when in Ohm mode.
pub const UUID_CHAR_I1: &str = "8ae89a2a-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_I2: &str = "8ae89bec-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_I3: &str = "8ae89dc2-ad7d-11e6-80f5-76304dec7eb7";
pub const UUID_CHAR_I4: &str = "8ae89f66-ad7d-11e6-80f5-76304dec7eb7";

#[derive(Debug)]
pub struct Device {
    pub peripheral: btleplug::platform::Peripheral,
    char_battery: Characteristic,
    pub char_m1: Characteristic,
    pub char_m2: Characteristic,
    char_i1: Characteristic,
    char_i2: Characteristic,
    char_i3: Characteristic,
    char_i4: Characteristic,
}

pub async fn open_device() -> Result<Device> {
    println!("Waiting for pairing.");

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters
        .into_iter()
        .next()
        .expect("No Bluetooth adapters found");

    let mut char_battery = None;
    let mut char_m1 = None;
    let mut char_m2 = None;
    let mut char_i1 = None;
    let mut char_i2 = None;
    let mut char_i3 = None;
    let mut char_i4 = None;
    let peripheral = 'outer: loop {
        central.start_scan(ScanFilter::default()).await?;
        sleep(Duration::from_secs(2)).await;

        let devices = central.peripherals().await?;
        for device in devices {
            let properties = device.properties().await?.unwrap();
            let Some(name) = properties.local_name else {
                continue;
            };

            if name == "BT Smart Controller" {
                break 'outer device;
            }
        }
    };

    peripheral.connect().await?;
    println!("Connected to BT Smart Controller");

    peripheral.discover_services().await?;
    let characteristics = peripheral.characteristics();
    for characteristic in characteristics.into_iter() {
        if characteristic.properties.contains(CharPropFlags::NOTIFY) {
            peripheral.subscribe(&characteristic).await?;
        }
        match &characteristic.uuid.to_string() as &str {
            UUID_CHAR_BATTERY => char_battery = Some(characteristic),
            UUID_CHAR_M1 => char_m1 = Some(characteristic),
            UUID_CHAR_M2 => char_m2 = Some(characteristic),
            UUID_CHAR_I1 => char_i1 = Some(characteristic),
            UUID_CHAR_I2 => char_i2 = Some(characteristic),
            UUID_CHAR_I3 => char_i3 = Some(characteristic),
            UUID_CHAR_I4 => char_i4 = Some(characteristic),
            _ => (),
        }
    }

    if char_battery.is_none()
        || char_m1.is_none()
        || char_m2.is_none()
        || char_i1.is_none()
        || char_i2.is_none()
        || char_i3.is_none()
        || char_i4.is_none()
    {
        bail!("Did not find one of the expected characteristics!");
    }

    Ok(Device {
        peripheral,
        char_battery: char_battery.unwrap(),
        char_m1: char_m1.unwrap(),
        char_m2: char_m2.unwrap(),
        char_i1: char_i1.unwrap(),
        char_i2: char_i2.unwrap(),
        char_i3: char_i3.unwrap(),
        char_i4: char_i4.unwrap(),
    })
}
