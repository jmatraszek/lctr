#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate wiringpi;

use wiringpi::pin::Value;
use wiringpi::pin::Value::{High, Low};
use std::{thread, time};
use std::collections::VecDeque;

extern crate mpd;

use mpd::Client;

/// LCTR
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "pin_number", default_value = "6", short = "p", long = "pin-number")]
    pub pin_number: u16,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Opt, log_level: verbosity| {
    let pi = wiringpi::setup();
    let pin = pi.input_pin(args.pin_number);
    let interval = time::Duration::from_millis(500);
    let mut initial_light_readings: VecDeque<_> = vec![High, High].into_iter().collect();
    let mut light_readings: VecDeque<Value> = VecDeque::with_capacity(2);
    light_readings.append(&mut initial_light_readings);

    let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    info!("MPD status: {:?}", conn.status());

    loop {
        let value = pin.digital_read();
        info!("DIGITAL READ: {:?}", value);
        light_readings.pop_front();
        light_readings.push_back(value);
        info!("{:?}", light_readings);
        match Vec::from(light_readings.clone()).as_slice() {
            &[High, Low] => start_playback(&mut conn)?,
            &[Low, High] => stop_playback(&mut conn)?,
            _ => (),
        };
        thread::sleep(interval);
    }
});

fn start_playback(conn: &mut Client) -> Result<()> {
    conn.clear()?;
    conn.volume(100)?;
    conn.load("wbgo", ..).unwrap();
    conn.play()?;
    info!("Playback started.");
    Ok(())
}

fn stop_playback(conn: &mut Client) -> Result<()> {
    conn.stop()?;
    conn.clear()?;
    info!("Playback stopped.");
    Ok(())
}
