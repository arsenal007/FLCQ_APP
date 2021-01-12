

use super::com;

impl SWidgets {
    pub fn plot()  {

            conrod::widget::Tabs::new(&[
                (ids.tab_frequency, "FREQUENCY"),
                (ids.tab_frequency_calibration, "F CALIBRATION"),
                (ids.tab_inductance, "L MEASURMENTS"),
                (ids.tab_capacitance, "C MEASURMENTS"),
                (ids.tab_crystal, "Q MEASURMENTS"),
            ])
            .h_of(ids.top)
            .parent(ids.top)
            .middle()
            .layout_horizontally()
            .color(conrod::color::WHITE)
            .label_color(conrod::color::BLACK)
            .starting_canvas(ids.tab_frequency)
            .label_font_size(50)
            .set(ids.tabs, ui);

            widget::Text::new("UART PORT: ")
                //.padded_w_of(ids.left_col, PAD)
                .mid_left_with_margin_on(ids.uart, 10.0)
                .color(conrod::color::BLACK)
                .font_size(35)
                .line_spacing(3.0)
                .set(ids.uart_label_port, ui);

            let ports = widget::DropDownList::new(&a, selected_uart_port)
                .scrollbar_next_to()
                .max_visible_items(1usize)
                .h_of(ids.uart)
                .w(300.0)
                .scrollbar_width(40.0)
                .color(conrod::color::WHITE) // conrod::color::YELLOW
                .label_font_size(35)
                .center_justify_label()
                .top_left_with_margins_on(ids.uart, 0.0, 300.0)
                .set(ids.uart_ports, ui);

            match ports {
                Some(id) => {
                    println!("id {}\n", id);
                    selected_uart_port = Some(id)
                }
                None => (),
            }
    }
}


struct StateMachine<S> {
     shared_widgets: SWidgets,
     state: S,
}

impl StateMachine<NotConnected> {
    fn new() -> Self {
        Self {
            
            state: NotConnected { flcq: com::init(),
                
            }
        }
    }
}

struct NotConnected {
    
}

struct Connected {
    flcq: com::Flcq,
}

struct FMesurmentInit {

}


struct Done{

}

enum StateMachineWrapper {
    NotConnected(StateMachine<NotConnected>),
    Connected(StateMachine<Connected>),
    FMesurmentInit(StateMachine<FMesurmentInit>), // check if frequency is calibrated
    Done(StateMachine<Done>),
}

pub struct Factory {
    machine: StateMachineWrapper,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            machine: StateMachineWrapper::NotConnected(StateMachine::new()),
        }
    }
}


impl From<StateMachine<NotConnected>> for StateMachine<Connected> {
    fn from(val: StateMachine<NotConnected>) -> StateMachine<Connected> {


        StateMachine {
                state: Connected {
                flcq: com::open(val.state.get_name())
            }
        }
    }
}

                if widget::Button::new()
                    .top_left_with_margins_on(ids.uart, 0.0, 600.0)
                    .h_of(ids.uart)
                    .w(350.0)
                    .label("Click to Connect")
                    .label_font_size(35)
                    .color(conrod::color::RED) //RED
                    .set(ids.uart_connect_button, ui)
                    .was_clicked()
                {
                    match selected_uart_port {
                        Some(id) => {
                            flcq = com::open(&list[id].port_name);
                            history.append()
                        }
                        None => (),
                    }
                }
