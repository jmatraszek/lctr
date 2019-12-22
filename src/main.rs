use structopt::StructOpt;
use sysfs_gpio::{Pin, Direction};
use std::{thread, time};
use std::collections::VecDeque;
use mpd::Client;
use chrono::prelude::*;
use std::error::Error;
#[macro_use]
extern crate log;

/// LCTR
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "pin_number", default_value = "6", short = "p", long = "pin-number")]
    pub pin_number: u64,
    #[structopt(name = "dry_run", short = "n", long = "dry-run")]
    pub dry_run: bool,
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();
    args.log.log_all(Some(args.verbosity.log_level()))?;
    debug!("Command line options: {:?}", args);

    let pin = Pin::new(args.pin_number);
    let interval = time::Duration::from_millis(500);
    let mut light_readings: VecDeque<u8> = VecDeque::with_capacity(2);
    light_readings.append(&mut vec![1, 1].into_iter().collect::<VecDeque<_>>());

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    info!("MPD status: {:?}", conn.status());

    pin.with_exported(|| {
        pin.set_direction(Direction::In)?;

        loop {
            let value = pin.get_value()?;
            trace!("Pin value: {:?}", value);
            light_readings.pop_front();
            light_readings.push_back(value);
            trace!("Light readings: {:?}", light_readings);
            match Vec::from(light_readings.clone()).as_slice() {
                &[1, 0] => start_playback(&args, &mut conn).unwrap(),
                &[0, 1] => stop_playback(&args, &mut conn).unwrap(),
                _ => trace!("No change."),
            };

            conn.ping().unwrap_or_else(|err| {
                error!("MPD ping failed with {} error.", err);
                conn.clearerror().unwrap();
            });

            thread::sleep(interval);
        }
    })?;
    Ok(())
}

fn start_playback(args: &Opt, conn: &mut Client) -> Result<(), Box<dyn Error>> {
    let settings = time_settings();
    if args.dry_run == false {
        conn.clear().unwrap_or_else(|err| {
            error!("Clear failed with {} error.", err);
        });
        conn.load(settings.0, ..).unwrap_or_else(|err| {
            error!("Playlist load failed with {} error.", err);
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

fn stop_playback(args: &Opt, conn: &mut Client) -> Result<(), Box<dyn Error>> {
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
