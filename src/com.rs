extern crate clap;
extern crate serialport;

//use std::io::{self, Write};
//use std::time::Duration;

//use serialport::prelude::*;

fn timeout<P: serialport::SerialPort, T: std::fmt::Display>(port: &P, s: &T) -> () {
    match port.name() {
        Some(name) => println!("{}: Timeout port \"{}\"", s, name),
        None => println!("\"{}\" port name is not avilable", s),
    }
}

fn eeprom_write_byte<P: serialport::SerialPort>(port: &P, address: &u8, data: &u8) -> () {
    let write_data = vec![0x03u8, *data, *address, 0xFFu8, 0xFFu8];

    match port.write(&write_data) {
        Ok(_) => {
            let mut read_data = vec![0; 5];
            match port.read(&mut read_data) {
                Ok(_n) => {
                    if read_data[0] == 0x04
                        && read_data[1] == *data
                        && read_data[2] == *address
                        && _n == 5
                    {
                        ()
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout(
                    port,
                    &std::string::String::from(" [eeprom write byte respond ] "),
                ),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout(
            port,
            &std::string::String::from(" [eeprom write byte request ] "),
        ),
        Err(e) => panic!("Error while writing data to the port: {}", e),
    };
}

fn eeprom_read_byte<P: serialport::SerialPort>(port: &P, adrress: &u8) -> u8 {
    let write_data = vec![0x05u8, *adrress, 0xFFu8, 0xFFu8];

    match port.write(&write_data) {
        Ok(_) => (),
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => timeout(
            port,
            &std::string::String::from(" [ eeprom read byte request ] "),
        ),
        Err(e) => eprintln!("{:?}", e),
    }
    let mut read_data = vec![0; 5];
    match port.read(&mut read_data) {
        Ok(_n) => {
            //println!("{} {} {}", read_data[0], read_data[1], read_data[2]);
            if read_data[0] == 0x04 && read_data[2] == *adrress && _n == 5 {
                read_data[1]
            } else {
                eprintln!("return address is different as in read command");
                0xFFu8
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
            timeout(port, &std::string::String::from(" [ eeprom read byte ] "));
            0xFFu8
        }
        Err(e) => {
            eprintln!("{:?}", e);
            0xFFu8
        }
    }
}

pub struct Flcq {
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl Flcq {
    fn new<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(port_name: &T) -> Self {
        let mut settings: serialport::SerialPortSettings = Default::default();
        settings.timeout = std::time::Duration::from_millis(100000);
        settings.baud_rate = 57600u32;
        match serialport::open_with_settings(&port_name, &settings) {
            Ok(result) => Flcq { port: Some(result) },
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                ::std::process::exit(1);
            }
        }
    }
}

impl Flcq {
    pub fn disconnect(&mut self) {
        self.port = None;
    }
}

impl Flcq {}

impl Flcq {}

impl Flcq {
    fn eeprom_write_f64(&mut self, _adrress: &u8, _value: &f64) -> () {
        let b = _value.clone();
        let _byte_array: f64;
        unsafe {
             _byte_array = std::mem::transmute::<f64, [u8; 8]>(b);
        }
            for (i, item) in _byte_array.iter().enumerate() {
                let adrress = *_adrress + i as u8;
                //println!("{} {}", i, item);
                //thread::sleep(std::time::Duration::from_millis(1000));
                self.eeprom_write_byte(&adrress, &item);
            }
        }
    }
}

impl Flcq {
    pub fn eeprom_read_f64(&mut self, _adrress: &u8) -> f64 {
        unsafe {
            let mut _byte_array = [0u8; 8];

            for i in 0..=7 {
                let adrress = *_adrress + i as u8;
                _byte_array[i] = self.eeprom_read_byte(&adrress);
            }
            std::mem::transmute::<[u8; 8], f64>(_byte_array)
        }
    }
}

impl Flcq {
    fn temperature(&self, _first: u8, _second: u8) -> f64 {
        let data = [_second, _first];
        unsafe {
            let raw = std::mem::transmute::<[u8; 2], u16>(data);
            let f = raw as f64;
            f * 0.0625
        }
    }
}

//temperature
impl Flcq {
    pub fn t(&mut self) -> f64 {
        let write_data = vec![0x09u8, 0x08u8, 0x00u8, 0xFFu8, 0xFFu8];
        let mut res: f64 = -100.0;
        match self.port.write(&write_data) {
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                &std::string::String::from(" [ query for temperature from FLCQ ] "),
            ),
            Err(e) => eprintln!("{:?}", e),
        };
        let mut read_data = vec![0; 5];
        match self.port.read(&mut read_data) {
            Ok(_n) => {
                if read_data[0] == 0x0A && _n == 5 {
                    res = self.temperature(read_data[1], read_data[2])
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                &std::string::String::from(" [ wait for temperature from FLCQ ] "),
            ),
            Err(e) => eprintln!("{:?}", e),
        }
        res
    }
}
impl Flcq {
    fn frequency(&self, prescaler: u8, tmr0: u8, overflows_array: [u8; 4]) -> f64 {
        let overflows: u32;
        unsafe {
            overflows = std::mem::transmute::<[u8; 4], u32>(overflows_array);
        }
        let prescaler_values = [1.0f64, 2.0f64, 4.0f64, 8.0f64, 16.0f64];
        println!(
            "{} {} {}",
            overflows,
            prescaler_values[(prescaler + 1u8) as usize],
            tmr0 as f64
        );
        prescaler_values[(prescaler + 1u8) as usize] * (256.0f64 * overflows as f64 + tmr0 as f64)
    }
}

impl Flcq {
    pub fn get_frequency_c(&mut self, n: u8) -> f64 {
        let mut freq: f64 = -10000.0f64;
        if (0 < n) && (n < 255) {
            let write_data = vec![0x0Bu8, 0x10u8, 0x00u8, n, 0xFFu8, 0xFFu8];
            match self.port.write(&write_data) {
                Ok(_) => (),
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                    &std::string::String::from(" [ query for frequency from FLCQ ] "),
                ),
                Err(e) => eprintln!("{:?}", e),
            };

            let mut read_data = vec![0; 9];

            match self.port.read(&mut read_data) {
                Ok(_n) => {
                    let n_overflow_tmp = [read_data[3], read_data[4], read_data[5], read_data[6]];
                    let overflows: u32;
                    unsafe {
                        overflows = std::mem::transmute::<[u8; 4], u32>(n_overflow_tmp);
                    }
                    println!("overflows {}", overflows);
                    if read_data[0] == 0x06 && _n == 9 {
                        let n_overflow = [read_data[3], read_data[4], read_data[5], read_data[6]];
                        freq = self.frequency(read_data[1], read_data[2], n_overflow);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                    &std::string::String::from(" [ wait for temperature from FLCQ ] "),
                ),
                Err(e) => eprintln!("{:?}", e),
            }
        } else {
            println!("wrong averging over {:?}, must be (0 < n < 255) ", n);
        }
        freq
    }
}

impl Flcq {
    fn get_frequency(&mut self, mut n: u8) -> f64 {
        if (0 < n) && (n < 255) {
            let write_data = vec![0x07u8, 0x10u8, 0x00u8, n, 0xFFu8, 0xFFu8];
            match self.port.write(&write_data) {
                Ok(_) => (),
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                    &std::string::String::from(" [ query for frequency from FLCQ ] "),
                ),
                Err(e) => eprintln!("{:?}", e),
            };

            let mut frequencies = Vec::new();
            loop {
                let mut read_data = vec![0; 9];
                match self.port.read(&mut read_data) {
                    Ok(_n) => {
                        if read_data[0] == 0x06 && _n == 9 {
                            let n_overflow =
                                [read_data[3], read_data[4], read_data[5], read_data[6]];
                            frequencies.push(self.frequency(
                                read_data[1],
                                read_data[2],
                                n_overflow,
                            ));
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => self.timeout(
                        &std::string::String::from(" [ wait for temperature from FLCQ ] "),
                    ),
                    Err(e) => eprintln!("{:?}", e),
                }
                n = n - 1;
                if n == 0 {
                    break;
                }
            }
            let sum = frequencies.iter().sum::<f64>() as f64;
            sum / frequencies.len() as f64
        } else {
            println!("wrong averging over {:?}, must be (0 < n < 255) ", n);
            -1000.0f64
        }
    }
}

pub fn ports() -> std::result::Result<std::vec::Vec<serialport::SerialPortInfo>, serialport::Error>
{
    serialport::available_ports()
}

pub fn open<T: std::fmt::Display + AsRef<std::ffi::OsStr> + ?Sized>(v: &T) -> Flcq {
    Flcq::new(v)
}
