extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup();

    //Use WiringPi pin 0 as input
    let pin0 = pi.input_pin(1);
    let pin2 = pi.input_pin(4);

    let interval = time::Duration::from_millis(500);

    loop {
        let digital_value = pin0.digital_read();
        let analog_value = pin2.analog_read();
        match digital_value {
            High => { println!("Digital: High") },
            Low => { println!("Digital: Low") },
        }
        println!("Analog: {}", analog_value);
        thread::sleep(interval);
    }
}
