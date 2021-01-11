pub struct TInductanceL {
    l_tab_c0l0: Option<(f64, f64)>,
    c_tab_c0l0: Option<(f64, f64)>,
}

impl TInductanceL {
    pub fn get_l(&mut self) -> &mut Option<(f64, f64)> {
        &mut self.l_tab_c0l0
    }
}

impl TInductanceL {
    pub fn get_c(&mut self) -> &mut Option<(f64, f64)> {
        &mut self.c_tab_c0l0
    }
}

impl TInductanceL {
    pub fn get(&self) -> Option<(f64, f64)> {
        if let Some(_) = self.l_tab_c0l0 {
            self.l_tab_c0l0.clone()
        } else if let Some(_) = self.c_tab_c0l0 {
            self.c_tab_c0l0.clone()
        } else {
            None
        }
    }
}

impl TInductanceL {
    pub fn reset(&mut self) -> () {
        self.l_tab_c0l0 = None;
        self.c_tab_c0l0 = None;
    }
}

impl Default for TInductanceL {
    fn default() -> Self {
        Self {
            l_tab_c0l0: None,
            c_tab_c0l0: None,
        }
    }
}
