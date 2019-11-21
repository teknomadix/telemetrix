extern crate tokio;
extern crate udev;

use std::error::Error;
use std::path::Path;
use tokio::net::UnixStream;
use tokio::prelude::*;

// TODO: use `failure` crate for error interop
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let context = udev::Context::new()?;
    let mut enumerator = udev::Enumerator::new(&context)?;
    enumerator.match_subsystem("tty")?;

    let socket_path = Path::new("/var/run/gpsd.sock");
    let mut socket = UnixStream::connect(&socket_path).wait()?;
    enumerator
        .scan_devices()?
        .filter(|device| {
            // add conditions here to detect different GPS devices
            is_gps_puck(&device)
        })
        .flat_map(|device| {
            device
                .properties()
                .find(|prop| prop.name() == "DEVNAME")
                .and_then(|prop| prop.value().to_os_string().into_string().ok())
                .or_else(|| {
                    eprintln!(
                        "WARN: Got GPS device at {:?} but couldn't get its name from props.",
                        device.syspath()
                    );
                    None
                })
        })
        .for_each(|dev_name| {
            println!("GPS device found: {:?}", dev_name);
            let mut cmd = String::from("+");
            cmd.push_str(&dev_name[..]);
            socket.write(&cmd.into_bytes()).unwrap_or_else(|err| {
                eprintln!("ERROR: failed to write device to gpsd: {:?}", err);
                0
            });
        });

    Ok(())
}

fn is_gps_puck(device: &udev::Device) -> bool {
    // u-blox AG, u-blox 7 [linux module: cdc_acm]
    let is_vendor = device
        .properties()
        .find(|prop| prop.name() == "ID_VENDOR_ID" && prop.value() == "1546")
        .is_some();
    let is_model = device
        .properties()
        .find(|prop| prop.name() == "ID_MODEL_ID" && prop.value() == "01a7")
        .is_some();
    is_vendor && is_model
}
