#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate sysfs_gpio;
use sysfs_gpio::Pin;

use std::{thread, time};
use std::collections::VecDeque;

extern crate mpd;
use mpd::Client;

extern crate chrono;
use chrono::prelude::*;

/// LCTR
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "pin_number", default_value = "6", short = "p", long = "pin-number")]
    pub pin_number: u64,
    #[structopt(name = "dry_run", short = "n", long = "dry-run")]
    pub dry_run: bool,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Opt, log_level: verbosity| {
    let pin = Pin::new(args.pin_number);
    let interval = time::Duration::from_millis(500);
    let mut initial_light_readings: VecDeque<_> = vec![1, 1].into_iter().collect();
    let mut light_readings: VecDeque<u8> = VecDeque::with_capacity(2);
    light_readings.append(&mut initial_light_readings);

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    info!("MPD status: {:?}", conn.status());

    pin.with_exported(|| {
        pin.set_direction(sysfs_gpio::Direction::In);

        loop {
            let value = pin.get_value()?;
            trace!("DIGITAL READ: {:?}", value);
            light_readings.pop_front();
            light_readings.push_back(value);
            trace!("{:?}", light_readings);
            match Vec::from(light_readings.clone()).as_slice() {
                &[1, 0] => start_playback(&args, &mut conn).unwrap(),
                &[0, 1] => stop_playback(&args, &mut conn).unwrap(),
                _ => trace!("No change."),
            };

            conn.ping().unwrap_or_else(|err| {
                conn.clearerror();
                error!("Ping failed with {} error.", err);
            });

            thread::sleep(interval);
        }
    });
});

fn start_playback(args: &Opt, conn: &mut Client) -> Result<()> {
    let settings = time_settings();
    if args.dry_run == false {
        conn.clear().unwrap_or_else(|err| {
            error!("Clear failed with {} error.", err);
        });
        conn.load(settings.0, ..).unwrap_or_else(|err| {
            error!("Load failed with {} error.", err);
        });
        conn.volume(settings.1).unwrap_or_else(|err| {
            error!("Volume failed with {} error.", err);
        });
        conn.play().unwrap_or_else(|err| {
            error!("Play failed with {} error.", err);
        });
    }
    info!("Playback started. Playlist: {}. Volume: {}.", settings.0, settings.1);
    Ok(())
}

fn stop_playback(args: &Opt, conn: &mut Client) -> Result<()> {
    if args.dry_run == false {
        conn.stop().unwrap_or_else(|err| {
            error!("Stop failed with {} error.", err);
        });
        conn.clear().unwrap_or_else(|err| {
            error!("Clear failed with {} error.", err);
        });
    }
    info!("Playback stopped.");
    Ok(())
}

fn time_settings() -> (&'static str, i8) {
    let time: DateTime<Local> = Local::now();
    let hour = time.hour();
    if hour > 0 && hour < 6 {
        ("WBGO", 20)
    } else {
        ("BBC - Radio 6music", 50)
    }
}
