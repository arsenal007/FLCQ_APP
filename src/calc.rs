use commands::TCommand;

pub struct CalculationL<'c> {
    pub c0l0: &'c mut Option<(f64, f64)>,
    pub f1: f64,
    pub f2: f64,
    pub c1: f64,
    pub c2: f64,
}

impl<'c> TCommand for CalculationL<'c> {
    fn execute(&mut self) -> () {
        if let None = self.c0l0 {
            *self.c0l0 = Some(Self::calc_l(self.f1, self.f2, self.c1, self.c2));
        }
    }
}

impl CalculationL<'_> {
    fn calc_l(f1: f64, f2: f64, c1: f64, c2: f64) -> (f64, f64) {
        let f1_2 = f1 * f1;
        let f2_2 = f2 * f2;

        let c1f = c1 / 1000_000_000.0; // in farad
        let c2f = c2 / 1000_000_000.0;

        let c = (f1_2 * c1 - f2_2 * c2) / (f2_2 - f1_2);

        let l = (1.0 / f1_2 - 1.0 / f2_2)
            / (4.0 * std::f64::consts::PI * std::f64::consts::PI * (c1f - c2f)); // in Henry
        (c, l * 1000_000_000.0) // return in pico farads and micro Henrys
    }
}
