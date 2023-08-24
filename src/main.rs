#[allow(unused_imports)]
use std::process::Command;
use udev::{MonitorBuilder, EventType};


fn main() {

    let mut monitor = MonitorBuilder::new().unwrap()
        .match_subsystem("drm").unwrap()
        .listen().unwrap();


    println!("Listening for HDMI plug events...");

    // Loop to listen for udev events
    for event in monitor.events() {
        // Check if the event is an HDMI connect event
        if event.event_type() == EventType::Change && event.devname().map_or(false, |name| name.contains("HDMI")) {
            println!("HDMI event detected!");

            // Check if HDMI is connected using xrandr
            let xrandr_output = Command::new("xrandr").output().expect("Failed to run xrandr");
            let str_output = String::from_utf8_lossy(&xrandr_output.stdout);

            if str_output.contains("HDMI1 connected") {
                // Run your xrandr command to change settings
                let output = Command::new("xrandr")
                    .arg("--output")
                    .arg("eDP-1")
                    .arg("--brightness")
                    .arg("0.9")
                    .output()
                    .expect("WTF happened");

                let str_output = String::from_utf8_lossy(&output.stdout);
                println!("{:?}", str_output);
            }
        }
    }
}

