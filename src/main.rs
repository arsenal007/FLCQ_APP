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
                        if read_data[0] == 0x04
                            && read_data[1] == *adrress
                            && read_data[2] == *data
                            && _n == 5
                        {
                            ()
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        match self.port.name() {
                            Some(name) => println!("Write: timeout port \"{}\"", name),
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

impl Flcq {
    fn eeprom_read_byte(&mut self, adrress: &u8) -> Result<u8, std::io::Error> {
        let write_data = vec![0x05u8, *adrress, 0xFFu8, 0xFFu8];

        self.port.write(&write_data)?;
        let mut read_data = vec![0; 5];
        match self.port.read(&mut read_data) {
            Ok(_n) => {
                if read_data[0] == 0x04 && read_data[1] == *adrress && _n == 5 {
                    Ok(read_data[2])
                } else {
                    let error = std::io::Error::new(
                        std::io::ErrorKind::AddrNotAvailable,
                        "return address is different as in read command",
                    );
                    Err(error)
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => match self.port.name() {
                Some(name) => {
                    println!("Read: Timeout port \"{}\"", name);
                    let error = std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout");
                    Err(error)
                }
                None => {
                    let error = std::io::Error::new(
                        std::io::ErrorKind::AddrNotAvailable,
                        "port name is not avilable",
                    );
                    Err(error)
                }
            },
            Err(e) => {
                eprintln!("{:?}", e);
                Err(e)
            }
        }
    }
}

impl Flcq {
    fn eeprom_write_f64(&mut self, _adrress: &u8, _value: &f64) -> () {
        unsafe {
            let b = _value.clone();
            let _byte_array = std::mem::transmute::<f64, [u8; 8]>(b);
            for (i, item) in _byte_array.iter().enumerate() {
                let adrress = *_adrress + i as u8;
                self.eeprom_write_byte(&adrress, &item);
            }
        }
    }
}

impl Flcq {
    fn eeprom_read_f64(&mut self, _adrress: &u8) -> f64 {
        unsafe {
            let mut _byte_array = [0u8; 8];

            for i in 0..=7 {
                let adrress = *_adrress + i as u8;
                match self.eeprom_read_byte(&adrress) {
                    Ok(value) => _byte_array[i] = value,
                    Err(_) => {}
                };
            }
            std::mem::transmute::<[u8; 8], f64>(_byte_array)
        }
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

    _flcq.eeprom_write_f64(&0u8, &200.0f64);
    println!("{:?}", _flcq.eeprom_read_f64(&0u8));
}
