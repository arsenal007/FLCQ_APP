extern crate clap;
extern crate serialport;

//use std::io::{self, Write};
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;

struct Flcq {
    port: Box<dyn serialport::SerialPort>,
}

impl Flcq {
    fn new<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(port_name: &T) -> Self {
        let mut settings: SerialPortSettings = Default::default();
        settings.timeout = Duration::from_millis(1000);
        settings.baud_rate = 57600u32;
        match serialport::open_with_settings(&port_name, &settings) {
            Ok(result) => Flcq { port: result },
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                ::std::process::exit(1);
            }
        }
    }
}

impl Flcq {
    fn eeprom_write_byte(&mut self, adrress: &u8, data: &u8) -> () {
        let write_data = vec![0x03u8, *adrress, *data, 0xFFu8, 0xFFu8];

        match self.port.write(&write_data) {
            Ok(_) => {
                let mut read_data = vec![0; 5];
                match self.port.read(&mut read_data) {
                    Ok(_n) => {
                        if read_data[2] == *data && read_data[1] == *adrress && _n == 5 {
                            ()
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        match self.port.name() {
                            Some(name) => println!("Timeout port \"{}\"", name),
                            None => (),
                        }
                        ()
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => panic!("Error while writing data to the port: {}", e),
        };
    }
}

fn main() {
    let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .get_matches();
    let mut _flcq = Flcq::new(matches.value_of("port").unwrap());
    _flcq.eeprom_write_byte(&0x01u8, &0xAAu8);
}
