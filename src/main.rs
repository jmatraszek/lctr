#[macro_use] extern crate quicli;
use quicli::prelude::*;

extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

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

    loop {
        let value = pin.digital_read();
        info!("Digital: {:?}", value);
        thread::sleep(interval);
    }
});
