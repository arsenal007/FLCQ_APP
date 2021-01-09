use eeprom::TEeprom as Eeprom;

pub trait TCommand {
    fn execute(&mut self) -> () {}
}

pub trait TCommandEeprom {
    fn execute(&mut self, _eeprom: &mut Eeprom) -> () {}
}

pub struct TSaveCref1 {
    pub value: f64,
}

impl TCommandEeprom for TSaveCref1 {
    fn execute(&mut self, eeprom: &mut Eeprom) -> () {
        eeprom.save_cref1(self.value);
    }
}

pub struct TSaveCref2 {
    pub value: f64,
}

impl TCommandEeprom for TSaveCref2 {
    fn execute(&mut self, eeprom: &mut Eeprom) -> () {
        eeprom.save_cref2(self.value);
    }
}

pub struct TSaveCL {
    pub value: (f64, f64),
}

impl TCommandEeprom for TSaveCL {
    fn execute(&mut self, eeprom: &mut Eeprom) -> () {
        eeprom.save_cl(self.value);
    }
}

pub struct MacroCommand<'b> {
    eeprom: Option<&'b mut Eeprom>,
    stack_eeprom: Vec<Box<dyn TCommandEeprom + 'b>>,
    stack: Vec<Box<dyn TCommand + 'b>>,
}

impl<'b> MacroCommand<'b> {
    pub fn new() -> Self {
        Self {
            stack_eeprom: Vec::new(),
            stack: Vec::new(),
            eeprom: None,
        }
    }

    pub fn append(&mut self, cmd: Box<dyn TCommandEeprom + 'b>) {
        self.stack_eeprom.push(cmd);
    }

    fn undo(&mut self) {
        self.stack.pop();
    }

    fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn eeprom(&mut self, e: &'b mut Eeprom) {
        self.eeprom = Some(e);
    }
}

impl Drop for MacroCommand<'_> {
    fn drop(&mut self) {
        for command in &mut self.stack {
            command.execute();
        }

        if let Some(&mut ref mut e) = self.eeprom {
            for command in &mut self.stack_eeprom {
                command.execute(e);
            }
        }
    }
}
