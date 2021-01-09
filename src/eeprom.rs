use super::com;

pub struct TEeprom {
    cref1: (Option<f64>, Option<u8>),
    cref2: (Option<f64>, Option<u8>),
    c0l0: (Option<(f64, f64)>, Option<u8>),
    init: bool,
    func: Vec<Box<dyn FnMut(&mut com::Flcq)>>,
}

impl TEeprom {
    const C0_OFFSET: u8 = 25u8; // f64
    const L0_OFFSET: u8 = Self::C0_OFFSET + 8u8; // f64
    const CL_N_OFFSET: u8 = Self::L0_OFFSET + 8u8; // u8
    const CREF1_AVG_OFFSET: u8 = Self::CL_N_OFFSET + 1u8;
    const CREF1_N_OFFSET: u8 = Self::CREF1_AVG_OFFSET + 8u8;
    const CREF2_AVG_OFFSET: u8 = Self::CREF1_N_OFFSET + 1u8;
    const CREF2_N_OFFSET: u8 = Self::CREF2_AVG_OFFSET + 8u8;
}

impl Default for TEeprom {
    fn default() -> Self {
        Self {
            cref1: (None, None),
            cref2: (None, None),
            c0l0: (None, None),
            init: true,
            func: Vec::new(),
        }
    }
}

impl TEeprom {
    pub fn c0(&self) -> Option<f64> {
        if let (Some((c0, _)), Some(_)) = self.c0l0 {
            return Some(c0);
        }
        None
    }

    pub fn c0_show(&self) -> Option<(f64, f64, u8)> {
        if let (Some((c0, l0)), Some(n)) = self.c0l0 {
            return Some((c0, l0, n));
        }
        None
    }

    pub fn cref1_show(&self) -> Option<(f64, u8)> {
        if let (Some(cref), Some(n)) = self.cref1 {
            return Some((cref, n));
        }
        None
    }
    pub fn cref2_show(&self) -> Option<(f64, u8)> {
        if let (Some(cref), Some(n)) = self.cref2 {
            return Some((cref, n));
        }
        None
    }
}

impl TEeprom {
    pub fn communicate(&mut self, flcq: &mut com::Flcq) -> () {
        if flcq.is_init() {
            for f in &mut self.func {
                (*f)(flcq);
            }
            self.func.drain(..);
            {
                Self::init_cref(
                    flcq,
                    &mut self.cref1,
                    (Self::CREF1_AVG_OFFSET, Self::CREF1_N_OFFSET),
                );
                Self::init_cref(
                    flcq,
                    &mut self.cref2,
                    (Self::CREF2_AVG_OFFSET, Self::CREF2_N_OFFSET),
                );

                if let (None, None) = self.c0l0 {
                    let c = flcq.eeprom_read_f64(&Self::C0_OFFSET);
                    let l = flcq.eeprom_read_f64(&Self::L0_OFFSET);
                    let n = flcq.eeprom_read_byte(&Self::CL_N_OFFSET);
                    if Self::reasonable_c0l0(&c, &l, &n) {
                        self.c0l0 = (Some((c, l)), Some(n));
                    } else {
                        self.c0l0 = (None, Some(0u8));
                    }
                }
            }
        }
    }
}

impl TEeprom {
    fn init_cref(
        flcq: &mut com::Flcq,
        cref: &mut (Option<f64>, Option<u8>),
        offsets: (u8, u8),
    ) -> () {
        let (avg_offset, n_offset) = offsets;
        if let (None, None) = cref {
            let n = flcq.eeprom_read_byte(&n_offset);
            let avg = flcq.eeprom_read_f64(&avg_offset);
            println!("avg {:?}, {:?}", avg, n);
            if Self::reasonable_c(&avg, &n) {
                *cref = (Some(avg), Some(n));
            } else {
                *cref = (None, Some(0u8));
            }
        }
    }
}

impl TEeprom {
    fn reasonable_c(avg: &f64, n: &u8) -> bool {
        (5.0 < *avg) && (*avg < 10001.0) && (0u8 < *n) && (*n < 255u8)
    }
}

impl TEeprom {
    fn reasonable_c0l0(c0: &f64, l0: &f64, n: &u8) -> bool {
        (0.0f64 < *c0)
            && (*c0 < 10001.0f64)
            && (0.1f64 < *l0)
            && (*l0 < 100.0f64)
            && (0u8 < *n)
            && (*n < 255u8)
    }
}

impl TEeprom {
    pub fn save_cref1(&mut self, cref: f64) {
        let c1 = self.cref1.clone();
        self.func.push(Box::new(move |flcq: &mut com::Flcq| {
            if let (Some(avg), Some(n)) = c1 {
                let m = n + 1u8;
                let n_avg = (avg * (n as f64) + cref) / (m as f64);
                flcq.eeprom_write_byte(&Self::CREF1_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::CREF1_AVG_OFFSET, &n_avg);
            } else if let (None, Some(n)) = c1 {
                let m = n + 1u8;
                flcq.eeprom_write_byte(&Self::CREF1_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::CREF1_AVG_OFFSET, &cref);
            }
        }));

        self.cref1 = (None, None);
    }
}

impl TEeprom {
    pub fn save_cref2(&mut self, cref: f64) {
        let c2 = self.cref2.clone();
        self.func.push(Box::new(move |flcq: &mut com::Flcq| {
            if let (Some(avg), Some(n)) = c2 {
                let m = n + 1u8;
                let n_avg = (avg * (n as f64) + cref) / (m as f64);
                flcq.eeprom_write_byte(&Self::CREF2_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::CREF2_AVG_OFFSET, &n_avg);
            } else if let (None, Some(n)) = c2 {
                let m = n + 1u8;
                flcq.eeprom_write_byte(&Self::CREF2_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::CREF2_AVG_OFFSET, &cref);
            }
        }));

        self.cref2 = (None, None);
    }
}

impl TEeprom {
    pub fn save_cl(&mut self, new_cl: (f64, f64)) {
        let cl = self.c0l0.clone();
        self.func.push(Box::new(move |flcq: &mut com::Flcq| {
            if let ((Some((avg_c, avg_l)), Some(n)), (c, l)) = (cl, new_cl) {
                let m = n + 1u8;
                let n_avg_c = (avg_c * (n as f64) + c) / (m as f64);
                let n_avg_l = (avg_l * (n as f64) + l) / (m as f64);
                flcq.eeprom_write_byte(&Self::CL_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::C0_OFFSET, &n_avg_c);
                flcq.eeprom_write_f64(&Self::L0_OFFSET, &n_avg_l);
            } else if let ((None, Some(n)), (c, l)) = (cl, new_cl) {
                let m = n + 1u8;
                flcq.eeprom_write_byte(&Self::CL_N_OFFSET, &m);
                flcq.eeprom_write_f64(&Self::C0_OFFSET, &c);
                flcq.eeprom_write_f64(&Self::L0_OFFSET, &l);
            }
        }));

        self.cref2 = (None, None);
    }
}
