use super::com;
use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use Ids;

struct PWidgets {
    tabs: widget::Id,
    top: widget::Id,
    tab_frequency: widget::Id,
    tab_frequency_calibration: widget::Id,
    tab_inductance: widget::Id,
    tab_capacitance: widget::Id,
    tab_crystal: widget::Id,
    uart: widget::Id,
    uart_label_port: widget::Id,
    uart_ports: widget::Id,
    selected_uart_port_index: Option<usize>,
    selected_port_name: std::string::String,
}

// permament widgets
impl PWidgets {
    pub fn plot(&self, ui: &mut conrod::UiCell) {
        let mut names_list = Vec::new();
        let list = com::ports().unwrap();
        for x in &list {
            names_list.push(x.port_name.clone());
        }

        conrod::widget::Tabs::new(&[
            (self.tab_frequency, "FREQUENCY"),
            (self.tab_frequency_calibration, "F CALIBRATION"),
            (self.tab_inductance, "L MEASURMENTS"),
            (self.tab_capacitance, "C MEASURMENTS"),
            (self.tab_crystal, "Q MEASURMENTS"),
        ])
        .h_of(self.top)
        .parent(self.top)
        .middle()
        .layout_horizontally()
        .color(conrod::color::WHITE)
        .label_color(conrod::color::BLACK)
        .starting_canvas(self.tab_frequency)
        .label_font_size(50)
        .set(self.tabs, ui);

        widget::Text::new("UART PORT: ")
            //.padded_w_of(ids.left_col, PAD)
            .mid_left_with_margin_on(self.uart, 10.0)
            .color(conrod::color::BLACK)
            .font_size(35)
            .line_spacing(3.0)
            .set(self.uart_label_port, ui);

        let ports = widget::DropDownList::new(&names_list, self.selected_uart_port_index)
            .scrollbar_next_to()
            .max_visible_items(1usize)
            .h_of(self.uart)
            .w(300.0)
            .scrollbar_width(40.0)
            .color(conrod::color::WHITE) // conrod::color::YELLOW
            .label_font_size(35)
            .center_justify_label()
            .top_left_with_margins_on(self.uart, 0.0, 300.0)
            .set(self.uart_ports, ui);

        match ports {
            Some(id) => {
                println!("id {}\n", id);
                self.selected_uart_port_index = Some(id);
                self.selected_port_name = names_list[id];
            }
            None => (),
        }
    }
}

impl PWidgets {
    fn port_name(&self) -> std::string::String {
        self.selected_port_name.clone()
    }
}

impl PWidgets {
    fn new(ids: &Ids) -> Self {
        Self {
            tabs: ids.tabs,
            top: ids.top,
            tab_frequency: ids.tab_frequency,
            tab_frequency_calibration: ids.tab_frequency_calibration,
            tab_inductance: ids.tab_inductance,
            tab_capacitance: ids.tab_capacitance,
            tab_crystal: ids.tab_crystal,
            uart: ids.uart,
            uart_label_port: ids.uart_label_port,
            uart_ports: ids.uart_ports,
            selected_uart_port_index: Some(0usize),
            selected_port_name: "".to_string(),
        }
    }
}

struct StateMachine<S> {
    permament_widgets: PWidgets,
    state: S,
    clicked: bool,
}

impl StateMachine<NotConnected> {
    fn new(ids: &Ids) -> Self {
        Self {
            permament_widgets: PWidgets::new(ids),
            state: NotConnected {
                uart: ids.uart,
                uart_connect_button: ids.uart_connect_button,
            },
            clicked: false,
        }
    }
}

impl StateMachine<NotConnected> {
    fn plot(&mut self, ui: &mut conrod::UiCell) -> () {
        self.permament_widgets.plot(ui);
        self.clicked = self.state.pushed_connect(ui);
    }
}

impl StateMachine<Connected> {
    fn plot(&mut self, ui: &mut conrod::UiCell) -> () {
        self.permament_widgets.plot(ui);
        self.clicked = self.state.pushed_connect(ui);
    }
}

struct NotConnected {
    uart: widget::Id,
    uart_connect_button: widget::Id,
}

impl NotConnected {
    pub fn pushed_connect(&mut self, ui: &mut conrod::UiCell) -> bool {
        widget::Button::new()
            .top_left_with_margins_on(self.uart, 0.0, 600.0)
            .h_of(self.uart)
            .w(350.0)
            .label("Click to Connect")
            .label_font_size(35)
            .color(conrod::color::RED) //RED
            .set(self.uart_connect_button, ui)
            .was_clicked()
    }
}

struct Connected {
    flcq: com::Flcq,
    uart: widget::Id,
    uart_connect_button: widget::Id,
}

impl Connected {
    pub fn pushed_connect(&mut self, ui: &mut conrod::UiCell) -> bool {
        widget::Button::new()
            .top_left_with_margins_on(self.uart, 0.0, 600.0)
            .h_of(self.uart)
            .w(350.0)
            .label("Click to Disconnect")
            .label_font_size(35)
            .color(conrod::color::GREEN)
            .set(self.uart_connect_button, ui)
            .was_clicked()
    }
}

enum StateMachineWrapper {
    NotConnected(StateMachine<NotConnected>),
    Connected(StateMachine<Connected>),
}

pub struct Factory {
    pub machine: StateMachineWrapper,
}

impl StateMachineWrapper {
    fn step(mut self) -> Self {
        match self {
            StateMachineWrapper::NotConnected(val) => StateMachineWrapper::Connected(val.into()),
            StateMachineWrapper::Connected(val) => StateMachineWrapper::NotConnected(val.into()),
        }
    }
}

impl StateMachineWrapper {
    pub fn plot(&mut self, ui: &mut conrod::UiCell) {
        match self {
            StateMachineWrapper::NotConnected(val) => val.plot(ui),
            StateMachineWrapper::Connected(val) => val.plot(ui),
        }
    }
}

impl From<StateMachine<NotConnected>> for StateMachine<Connected> {
    fn from(val: StateMachine<NotConnected>) -> StateMachine<Connected> {
        let s = val.permament_widgets.port_name();
        StateMachine {
            state: Connected {
                flcq: com::open(&s),
                uart: val.state.uart,
                uart_connect_button: val.state.uart_connect_button,
            },
            permament_widgets: val.permament_widgets,
            clicked: false,
        }
    }
}

impl From<StateMachine<Connected>> for StateMachine<NotConnected> {
    fn from(val: StateMachine<Connected>) -> StateMachine<NotConnected> {
        let s = val.permament_widgets.port_name();
        StateMachine {
            state: NotConnected {
                uart: val.state.uart,
                uart_connect_button: val.state.uart_connect_button,
            },
            permament_widgets: val.permament_widgets,
            clicked: false,
        }
    }
}

impl Factory {
    pub fn new(ids: &Ids) -> Self {
        Self {
            machine: StateMachineWrapper::NotConnected(StateMachine::new(ids)),
        }
    }

    pub fn clicked(&mut self) {
        match self.machine {
            StateMachineWrapper::NotConnected(val) if val.clicked => {
                self.machine = self.machine.step()
            }

            StateMachineWrapper::Connected(val) if val.clicked => {
                self.machine = self.machine.step()
            }
        }
    }
}
