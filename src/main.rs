extern crate clap;
extern crate dirs;
extern crate serialport;

//use std::io::{self, Write};

#[macro_use]
extern crate conrod;

use clap::{App, AppSettings, Arg};
use conrod::backend::glium::glium::{self, Surface};
use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
mod com;
mod commands;
mod eeprom;
mod sm;

use commands::TCommand;

use eeprom::TEeprom as Eeprom;

conrod::widget_ids! {
    struct Ids {
        master,
        top,
        uart,
        error,
        uart_label_port,
        uart_ports,
        uart_connect_button,
        uart_led,
        error_label,
        right_col,
        left_text,
        middle_text,
        right_text,
        text,
        refresh,
        tab_frequency,
        tab_frequency_calibration,
        tab_capacitance,
        capacitance_cref1_label,
        capacitance_cref1_edit,
        capacitance_cref1_pf,
        capacitance_e_cref1_label,
        capacitance_e_cref1_value,
        capacitance_e_cref1_pf,
        capacitance_input_toggle,
        capacitance_cref2_label,
        capacitance_cref2_edit,
        capacitance_cref2_pf,
        capacitance_e_cref2_label,
        capacitance_e_cref2_value,
        capacitance_e_cref2_pf,
        capacitance_eeprom_toggle,
        capacitance_input_label,
        capacitance_eeprom_label,
        capacitance_input_l_label,
        capacitance_eeprom_l_label,
        capacitance_lref1_toggle,
        capacitance_lref2_toggle,
        capacitance_eeprom_lc_label,
        capacitance_measure_button,
        capacitance_f1_label,
        capacitance_f2_label,
        capacitance_frequency_calibration_temperature_f1_label,
        capacitance_frequency_delta_temperature_f1_label,
        capacitance_current_temperature_f1_label,
        capacitance_calibration_temperature_f1_label,
        capacitance_delta_temperature_f1_label,
        capacitance_current_temperature_f2_label,
        capacitance_calibration_temperature_f2_label,
        capacitance_delta_temperature_f2_label,
        capacitance_c_label,
        capacitance_save_c1_button,
        capacitance_save_c2_button,
        tab_inductance,
        tab_crystal,
        label_frequency,
        label_frequency_calibration,
        tabs,
        count_frequency_slider,
        count_label_info,
        count_label,
        count_label_approx_in_sec,
        ref_frequency,
        ref_frequency_1,
        ref_frequency_2,
        freq_calibration_measure_button,
        freq_calibration_save_button,
        freq_calibration_temperature,
        freq_calibration_period,
        frequency_measure_button,
        frequency_temperature,
        inductance_measure_button,
        inductance_cref1_label,
        inductance_cref1_edit,
        inductance_cref1_pf,
        inductance_e_cref1_label,
        inductance_e_cref1_value,
        inductance_e_cref1_pf,
        inductance_input_toggle,
        inductance_cref2_label,
        inductance_cref2_edit,
        inductance_cref2_pf,
        inductance_e_cref2_label,
        inductance_e_cref2_value,
        inductance_e_cref2_pf,
        inductance_eeprom_toggle,
        inductance_input_label,
        inductance_eeprom_label,
        inductance_f1_label,
        inductance_f2_label,
        inductance_l_label,
        inductance_c_label,
        inductance_results_label,
        inductance_save_button,
        inductance_reset_button,
        inductance_current_temperature_f1_label,
        inductance_calibration_temperature_f1_label,
        inductance_delta_temperature_f1_label,
        inductance_current_temperature_f2_label,
        inductance_calibration_temperature_f2_label,
        inductance_delta_temperature_f2_label,
    }
}

type FpackT = (
    Option<(f64, f64, ((Option<f64>, std::string::String), f64))>,
    std::string::String,
);

/*pub trait TEReceiver {
    fn execute(&mut self) -> ();
}*/

pub trait TClicked {
    fn clicked(&mut self, button_label: &str, ui: &mut conrod::UiCell) -> bool;
    fn parent_id(&mut self) -> widget::Id;
}

pub struct ActionButton {
    parent_id: widget::Id,
    button_id: widget::Id,
    enabled: bool,
}

pub struct BToggle {
    parent_id: widget::Id,
    input_id: widget::Id,
    eeprom_id: widget::Id,
    input_label_id: widget::Id,
    eeprom_label_id: widget::Id,
    cref_input_active: bool,
    cref_eeprom_active: bool,
    inited_ids: bool,
}

pub struct InputCRef {
    toggle: BToggle,
    icref1: Option<f64>,
    icref2: Option<f64>,
    ecref1: Option<f64>,
    ecref2: Option<f64>,
    tab_id: std::collections::HashMap<
        std::string::String,
        (
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
            widget::Id,
        ),
    >,
}

pub struct TwoFreq<'a> {
    ab: Box<dyn TClicked + 'a>,
    f1: FpackT,
    f2: FpackT,
    error: widget::Id,
    error_label: widget::Id,
    f_label: [widget::Id; 2],
    current_temperature_f_label: [widget::Id; 2],
    calibration_temperature_f_label: [widget::Id; 2],
    delta_temperature_f_label: [widget::Id; 2],
}

fn main() {
    //let mut flcq: com::Flcq = com::init();
    let mut eeprom = Box::new(Eeprom::default());
    //_flcq.eeprom_write_f64(&0u8, &128.0f64);
    //println!("{:?}", _flcq.eeprom_read_f64(&0u8));
    //let t = _flcq.get_temperature();
    //_flcq.eeprom_write_f64(&8u8, &t);
    //let period = _flcq.get_frequency_c(254u8) / 3000000.0f64;
    //let period = _flcq.eeprom_read_f64(&16u8);

    //let period = flcq.eeprom_read_f64(&40u8);
    //let f = flcq.get_frequency_c(254u8) / period;

    //_flcq.eeprom_write_f64(&40u8, &period);
    //let t = _flcq.eeprom_read_f64(&8u8);
    //println!(        "measurments period {:?}sec, calibration temperature {}, current temperature {}",        period,        t,        _flcq.get_temperature()    );
    //println!("frequency {:?}Hz", _flcq.get_frequency_c(254u8) / period);

    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 480;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_title("FLCQ");

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let home_dir = dirs::home_dir().unwrap();
    //let font_folder = find_folder::Search::KidsThenParents(100, 100)        .for_folder("Noto-hinted")        .unwrap();
    let rdir = home_dir.to_str().unwrap();
    ui.fonts
        .insert_from_file("C:\\Windows\\Fonts\\timesbd.ttf")
        .unwrap();

    let ids = Ids::new(ui.widget_id_generator());
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let list = com::ports().unwrap();

    let mut a = Vec::new();

    let mut selected_uart_port: std::option::Option<usize> = Some(0usize);

    for x in &list {
        a.push(x.port_name.clone());
    }

    let frequency_count_intervals = (1.0, 254.0);

    let mut f_ref = None;

    let mut lref_input_active = true;
    let mut lref_eeprom_active = false;
    let mut fc = (254u8, (None, "".to_string()), (None, "".to_string()));
    let mut frequency = (None, "".to_string());

    let mut cref_source = InputCRef::default();
    cref_source.init(&ids);
    let mut f1_f2 = TwoFreq::new(
        Box::new(ActionButton::default()),
        ids.error,
        ids.error_label,
    );

    let mut sm = sm::Factory::new();

    'render: loop {
        // Handle all events.
        let mut events = Vec::new();

        events_loop.poll_events(|event| events.push(event));
        if events.is_empty() {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'render,
                    _ => (),
                },
                _ => (),
            }

            // Use the `winit` backend feature to convert the winit event to a conrod input.
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            // Handle the input with the `Ui`.
            ui.handle_event(input);

            let ui = &mut ui.set_widgets();

            //let s = period.to_string() + "Sec";

            // Our `Canvas` tree, upon which we will place our text widgets.
            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.top,
                        widget::Canvas::new()
                            .color(conrod::color::WHITE)
                            .length_weight(0.9)
                            .pad(0.0),
                    ),
                    (
                        ids.uart,
                        widget::Canvas::new()
                            .color(conrod::color::WHITE)
                            .length_weight(0.1)
                            .pad(0.0),
                    ),
                    (
                        ids.error,
                        widget::Canvas::new()
                            .color(conrod::color::WHITE)
                            .length_weight(0.1)
                            .pad(0.0),
                    ),
                ])
                .floating(true)
                .set(ids.master, ui);

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

            let mut history = commands::MacroCommand::new();
            eeprom.communicate(&mut flcq);

            if flcq.is_init() {
                if widget::Button::new()
                    .top_left_with_margins_on(ids.uart, 0.0, 600.0)
                    .h_of(ids.uart)
                    .w(350.0)
                    .label("Click to Disconnect")
                    .label_font_size(35)
                    .color(conrod::color::GREEN)
                    .set(ids.uart_connect_button, ui)
                    .was_clicked()
                {
                    flcq.disconnect();
                }
            } else {
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
            }

            {
                let (min, max) = frequency_count_intervals;
                let (count, _, _) = &mut fc;
                for value in widget::Slider::new(*count as f64, min, max)
                    .color(color::LIGHT_BLUE)
                    .h(60.0)
                    .mid_bottom_with_margin_on(ids.top, 5.0)
                    .w_of(ids.tab_frequency_calibration)
                    .parent(ids.tab_frequency_calibration)
                    .set(ids.count_frequency_slider, ui)
                {
                    //println!("start {}", value);
                    let value: f64 = value;
                    *count = value.round() as u8;
                }
            }

            widget::Text::new("Period ticks: ")
                .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 20.0)
                .color(conrod::color::BLACK)
                .right_justify()
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_frequency_calibration)
                .set(ids.count_label_info, ui);
            {
                let (count, _, _) = &fc;
                widget::Text::new(&format!("{:}", count))
                    .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 350.0)
                    .color(conrod::color::BLACK)
                    .right_justify()
                    .font_size(45)
                    .line_spacing(3.0)
                    .parent(ids.tab_frequency_calibration)
                    .set(ids.count_label, ui);

                let text = " [aprox. ".to_string();
                let pp = format!("{:.5}", (*count as f64) * 0.1048576);
                let text = text + &pp;
                let text = text + " Sec ]";

                widget::Text::new(&text)
                    .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 480.0)
                    .color(conrod::color::BLACK)
                    .font_size(45)
                    .line_spacing(3.0)
                    .parent(ids.tab_frequency_calibration)
                    .set(ids.count_label_approx_in_sec, ui);
            }
            widget::Text::new(&"Reference frequency: ".to_string())
                .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0 + 100.0, 20.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(1.0)
                .set(ids.ref_frequency_1, ui);

            match f_ref.clone() {
                Some(f) => {
                    if f < 1000.0 {
                        mhz_lebel(ui, &ids, "Hz".to_string());
                        let r = freq_show(ui, &ids, format!("{:.2}", f));
                        match r {
                            Some(hz) => f_ref = Some(hz),
                            None => (),
                        }
                    } else if 1000.0 < f && f < 1000_000.0 {
                        mhz_lebel(ui, &ids, "kHz".to_string());
                        let r = freq_show(ui, &ids, format!("{:.5}", f / 1000.0));
                        match r {
                            Some(k_hz) => f_ref = Some(k_hz * 1000.0),
                            None => (),
                        }
                    } else {
                        mhz_lebel(ui, &ids, "MHz".to_string());
                        let r = freq_show(ui, &ids, format!("{:.8}", f / 1000_000.0));
                        match r {
                            Some(m_hz) => f_ref = Some(m_hz * 1000_000.0),
                            None => (),
                        }
                    }
                }
                None => f_ref = Some(1000000.0),
            }

            if widget::Button::new()
                .w_h(250.0, 100.0)
                .bottom_left_with_margins_on(ids.tab_frequency_calibration, 450.0, 750.0)
                .label_font_size(50)
                .enabled(flcq.is_init())
                .label("Measure")
                .parent(ids.tab_frequency_calibration)
                .set(ids.freq_calibration_measure_button, ui)
                .was_clicked()
            {
                if flcq.is_init() {
                    let (count, _, _) = fc;
                    fc = (count, flcq.get_frequency_c(&count), flcq.t());
                }
            }

            if widget::Button::new()
                .w_h(250.0, 100.0)
                .bottom_left_with_margins_on(ids.tab_frequency_calibration, 450.0 - 120.0, 750.0)
                .label_font_size(50)
                .enabled(flcq.is_init())
                .hover_color(conrod::color::YELLOW)
                .press_color(conrod::color::RED)
                .label("Save")
                .set(ids.freq_calibration_save_button, ui)
                .was_clicked()
            {
                if flcq.is_init() {
                    match &fc {
                        (c, (Some(f), _), (Some(t), _)) => {
                            flcq.eeprom_write_byte(&0u8, &c); // save N count
                            flcq.eeprom_write_f64(&1u8, &f);
                            match f_ref {
                                Some(ref_frequency) => {
                                    let periode = f / ref_frequency;
                                    flcq.eeprom_write_f64(&9u8, &periode);
                                    flcq.eeprom_write_f64(&17u8, &t)
                                }
                                None => (),
                            }
                        }
                        (_, (None, _), _) => (),
                        (_, (Some(_), _), (None, _)) => (),
                    }
                }
            }

            let (_, frequency_cal, temperature_cal) = &fc;

            match (&frequency_cal, &f_ref) {
                ((Some(f), _), Some(ref_frequency)) => {
                    widget::Text::new(&format!("Mesured Period: {:.5} Sec", f / ref_frequency))
                        .bottom_left_with_margins_on(ids.tab_frequency_calibration, 480.0, 20.0)
                        .color(conrod::color::BLACK)
                        .right_justify()
                        .font_size(45)
                        .line_spacing(3.0)
                        .set(ids.freq_calibration_period, ui);
                }
                ((None, str), _) => widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(ids.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(ids.error_label, ui),
                ((Some(_), _), None) => (),
            }

            match &temperature_cal {
                (Some(t), _) => widget::Text::new(&format!("Temperature: {:.2} C", t))
                    .bottom_left_with_margins_on(ids.tab_frequency_calibration, 480.0 - 120.0, 20.0)
                    .color(conrod::color::BLACK)
                    .right_justify()
                    .font_size(45)
                    .line_spacing(3.0)
                    .set(ids.freq_calibration_temperature, ui),
                (None, str) => widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(ids.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(ids.error_label, ui),
            }

            if widget::Button::new()
                .w_h(250.0, 100.0)
                .bottom_left_with_margins_on(ids.tab_frequency, 450.0, 750.0)
                .label_font_size(50)
                .enabled(flcq.is_init())
                .label("Measure")
                .parent(ids.tab_frequency)
                .set(ids.frequency_measure_button, ui)
                .was_clicked()
            {
                if flcq.is_init() {
                    frequency = frequency_pack(&mut flcq);
                }
            }

            match &frequency {
                (Some((f, p, t)), _) => {
                    widget::Text::new(&format!("FREQ: {:.2} Hz", f / p))
                        .bottom_left_with_margins_on(ids.tab_frequency, 480.0, 20.0)
                        .color(conrod::color::BLACK)
                        .right_justify()
                        .font_size(45)
                        .line_spacing(3.0)
                        .set(ids.label_frequency, ui);

                    let (t1, tc) = t;
                    match &t1 {
                        (Some(t1), _) => temperature(
                            ui,
                            &ids,
                            ids.tab_frequency,
                            ids.frequency_temperature,
                            (*t1, *tc, *t1 - *tc),
                        ),
                        (None, str) => error(ui, &ids, &str),
                    };
                }
                (None, str) => error(ui, &ids, &str),
            }

            /*
            match &temperature {
                (Some(t), _) => {
                    widget::Text::new(&format!("Temperature: {:.2} C", t))
                        .bottom_left_with_margins_on(ids.tab_frequency, 300.0, 20.0)
                        .color(conrod::color::BLACK)
                        .right_justify()
                        .font_size(45)
                        .line_spacing(3.0)
                        .set(ids.frequency_temperature, ui);

                    widget::Text::new(&format!(
                        "Calibration Temperature: {:.2} C",
                        calibration_temperature
                    ))
                    .bottom_left_with_margins_on(ids.tab_frequency, 200.0, 20.0)
                    .color(conrod::color::BLACK)
                    .font_size(45)
                    .line_spacing(1.0)
                    .set(ids.frequency_saved_temperature, ui);
                }

                (None, str) => widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(ids.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(ids.error_label, ui),
            }*/

            // ====================================================================================
            // tab inductance
            // ====================================================================================
            cref_source.update_inductance_tab_cref(&eeprom, ui);

            /*{
                if let ((Some(f1), _str1), (Some(f2), _str2)) =
                    (frequency1_l.clone(), frequency2_l.clone())
                {
                    let (f1_, p1_, _) = f1;
                    let (f2_, p2_, _) = f2;
                    if swap_f(f1_ / p1_, f2_ / p2_) {
                        let a = frequency2_l.clone();
                        let b = frequency1_l.clone();
                        frequency1_l = a;
                        frequency2_l = b;
                    }
                }
            }

            let mut l_ab = ActionButton {
                parent_id: ids.tab_inductance,
                button_id: ids.inductance_measure_button,
                enabled: || flcq.is_init(),
            };*/

            f1_f2.new_tab(
                Box::new(ActionButton {
                    parent_id: ids.tab_inductance,
                    button_id: ids.inductance_measure_button,
                    enabled: flcq.is_init(),
                }),
                ids.inductance_f1_label,
                ids.inductance_current_temperature_f1_label,
                ids.inductance_calibration_temperature_f1_label,
                ids.inductance_delta_temperature_f1_label,
                ids.inductance_f2_label,
                ids.inductance_current_temperature_f2_label,
                ids.inductance_calibration_temperature_f2_label,
                ids.inductance_delta_temperature_f2_label,
            );
            match f1_f2.show(ui) {
                EClick::FNONE => (),
                EClick::F1 => f1_f2.f1_set(frequency_pack(&mut flcq)),
                EClick::F2 => f1_f2.f2_set(frequency_pack(&mut flcq)),
                EClick::END(f1, f2) => {
                    if let (Some(c1), Some(c2)) = cref_source.crefs() {
                        let (c, l) = calc_l(f1, f2, c1, c2);
                        /*widget::Text::new("Results: ")
                        .top_left_with_margins_on(ids.tab_inductance, 320.0, 70.0)
                        .color(conrod::color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_results_label, ui);*/

                        widget::Text::new(&format!("L: {:.2} uH", l))
                            .bottom_left_with_margins_on(ids.tab_inductance, 120.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(35)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_l_label, ui);

                        widget::Text::new(&format!("C: {:.2} pF", c))
                            .bottom_left_with_margins_on(ids.tab_inductance, 120.0, 800.0 - 530.0)
                            .color(conrod::color::BLACK)
                            .font_size(35)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_c_label, ui);

                        if widget::Button::new()
                            .w_h(220.0, 70.0)
                            .bottom_left_with_margins_on(ids.tab_inductance, 100.0, 800.0 - 250.0)
                            .label("SAVE")
                            .label_font_size(35)
                            .color(conrod::color::LIGHT_BLUE)
                            .set(ids.inductance_save_button, ui)
                            .was_clicked()
                        {
                            history.append(Box::new(commands::TSaveCL { value: (c, l) }));
                        }
                    }
                }
            }

            /*match (frequency1_l.clone(), frequency2_l.clone()) {
                ((None, str), _) => {
                    if l_ab.clicked("F1", ui) {
                        frequency1_l = frequency_pack(&mut flcq);
                    };

                    widget::Text::new(&str)
                        .color(conrod::color::BLACK)
                        .top_left_with_margins_on(ids.error, 5.0, 5.0)
                        .right_justify()
                        .font_size(16)
                        .line_spacing(3.0)
                        .set(ids.error_label, ui);
                }

                ((Some(f1), _), (None, str)) => {
                    if l_ab.clicked("F2", ui) {
                        frequency2_l = frequency_pack(&mut flcq);
                    };

                    let (f, p, t) = f1;

                    widget::Text::new(&format!("F1: {:.2} Hz", f / p))
                        .top_left_with_margins_on(ids.tab_inductance, 150.0, 20.0)
                        .color(conrod::color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_f1_label, ui);

                    let (current, calibration_temperature) = t;
                    if let (Some(t98), _) = current {
                        widget::Text::new(&format!("current temperature: {:.2} C", t98))
                            .top_left_with_margins_on(ids.tab_inductance, 180.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_current_temperature_f1_label, ui);

                        widget::Text::new(&format!(
                            "calibration temperature: {:.2} C",
                            calibration_temperature
                        ))
                        .top_left_with_margins_on(ids.tab_inductance, 210.0, 20.0)
                        .color(conrod::color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .parent(ids.tab_inductance)
                        .set(
                            ids.inductance_frequency_calibration_temperature_f1_label,
                            ui,
                        );

                        widget::Text::new(&format!(
                            "delta: {:.2} C",
                            t98 - calibration_temperature
                        ))
                        .top_left_with_margins_on(ids.tab_inductance, 240.0, 20.0)
                        .color(conrod::color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_frequency_delta_temperature_f1_label, ui);
                    }

                    widget::Text::new(&str)
                        .color(conrod::color::BLACK)
                        .top_left_with_margins_on(ids.error, 5.0, 5.0)
                        .right_justify()
                        .font_size(16)
                        .line_spacing(3.0)
                        .set(ids.error_label, ui);
                }

                ((Some(f1), _str1), (Some(f2), _str2)) => {
                    if let (Some(c1), Some(c2)) = cref_source.crefs() {
                        let (f1_, p1_, t1) = f1;
                        let (f2_, p2_, t2) = f2;

                        let f1 = f1_ / p1_;
                        let f2 = f2_ / p2_;

                        let (c, l) = calc_l(f1, f2, c1, c2);

                        widget::Text::new(&format!("F1: {:.2} Hz", f1))
                            .top_left_with_margins_on(ids.tab_inductance, 150.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_f1_label, ui);

                        let (current1, calibration_temperature1) = t1;
                        if let (Some(t98), _) = current1 {
                            widget::Text::new(&format!("current temperature: {:.2} C", t98))
                                .top_left_with_margins_on(ids.tab_inductance, 180.0, 20.0)
                                .color(conrod::color::BLACK)
                                .font_size(25)
                                .line_spacing(3.0)
                                .parent(ids.tab_inductance)
                                .set(ids.inductance_current_temperature_f1_label, ui);

                            widget::Text::new(&format!(
                                "calibration temperature: {:.2} C",
                                calibration_temperature1
                            ))
                            .top_left_with_margins_on(ids.tab_inductance, 210.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(
                                ids.inductance_frequency_calibration_temperature_f1_label,
                                ui,
                            );

                            widget::Text::new(&format!(
                                "delta: {:.2} C",
                                t98 - calibration_temperature1
                            ))
                            .top_left_with_margins_on(ids.tab_inductance, 240.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_frequency_delta_temperature_f1_label, ui);
                        }

                        widget::Text::new(&format!("F2: {:.2} Hz", f2))
                            .top_left_with_margins_on(ids.tab_inductance, 150.0, 560.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_f2_label, ui);

                        let (current2, calibration_temperature2) = t2;
                        if let (Some(t99), _) = current2 {
                            widget::Text::new(&format!("current temperature: {:.2} C", t99))
                                .top_left_with_margins_on(ids.tab_inductance, 180.0, 560.0)
                                .color(conrod::color::BLACK)
                                .font_size(25)
                                .line_spacing(3.0)
                                .parent(ids.tab_inductance)
                                .set(ids.inductance_current_temperature_f2_label, ui);

                            widget::Text::new(&format!(
                                "calibration temperature: {:.2} C",
                                calibration_temperature2
                            ))
                            .top_left_with_margins_on(ids.tab_inductance, 210.0, 560.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(
                                ids.inductance_frequency_calibration_temperature_f2_label,
                                ui,
                            );

                            widget::Text::new(&format!(
                                "delta: {:.2} C",
                                t99 - calibration_temperature2
                            ))
                            .top_left_with_margins_on(ids.tab_inductance, 240.0, 560.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_frequency_delta_temperature_f2_label, ui);
                        }

                        widget::Text::new("Results: ")
                            .top_left_with_margins_on(ids.tab_inductance, 320.0, 70.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_results_label, ui);

                        widget::Text::new(&format!("L: {:.2} uH", l))
                            .top_left_with_margins_on(ids.tab_inductance, 370.0, 20.0)
                            .color(conrod::color::BLACK)
                            .font_size(35)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_l_label, ui);

                        widget::Text::new(&format!("C: {:.2} pF", c))
                            .top_left_with_margins_on(ids.tab_inductance, 370.0, 420.0)
                            .color(conrod::color::BLACK)
                            .font_size(35)
                            .line_spacing(3.0)
                            .parent(ids.tab_inductance)
                            .set(ids.inductance_c_label, ui);

                        if l_ab.clicked("CLEAR", ui) {
                            frequency1_l = (None, "".to_string());
                            frequency2_l = (None, "".to_string());
                        }

                        if widget::Button::new()
                            .top_left_with_margins_on(ids.tab_inductance, 420.0, 510.0)
                            .h(30.0)
                            .w(350.0)
                            .label("Save to EEPROM")
                            .label_font_size(25)
                            .color(conrod::color::LIGHT_BLUE)
                            .set(ids.inductance_save_button, ui)
                            .was_clicked()
                        {
                            flcq.eeprom_write_f64(&25u8, &c);
                            flcq.eeprom_write_f64(&33u8, &l);
                        }
                    }
                }
            }*/
            // ====================================================================================
            // tab C
            // ====================================================================================
            cref_source.update_capacitance_tab_cref(&eeprom, ui);

            /*           match (cref_input_active, cref_eeprom_active) {
                            (true, false) => (),
                            (false, true) => (),
                            (true, true) => cref_input_active = false,
                            (false, false) => cref_eeprom_active = true,
                        }

                        for v in &mut widget::Toggle::new(cref_input_active)
                            .top_left_with_margins_on(ids.tab_capacitance, 30.0, 10.0)
                            .parent(ids.tab_capacitance)
                            .enabled(true)
                            .color(conrod::color::GREEN)
                            .border(4.0)
                            .border_color(conrod::color::RED)
                            .w(30.0)
                            .set(ids.capacitance_cref1_toggle, ui)
                        {
                            let n = v.clone();
                            if n {
                                cref_input_active = true;
                                cref_eeprom_active = false;
                            } else {
                                cref_input_active = false;
                                cref_eeprom_active = true;
                            }
                        }

                        for v in &mut widget::Toggle::new(cref_eeprom_active)
                            .top_left_with_margins_on(ids.tab_capacitance, 30.0, 550.0)
                            .parent(ids.tab_capacitance)
                            .enabled(true)
                            .color(conrod::color::GREEN)
                            .border(4.0)
                            .border_color(conrod::color::RED)
                            .w(30.0)
                            .set(ids.capacitance_cref2_toggle, ui)
                        {
                            let n = v.clone();
                            if n {
                                cref_input_active = false;
                                cref_eeprom_active = true;
                            } else {
                                cref_input_active = true;
                                cref_eeprom_active = false;
                            }
                        }

                        widget::Text::new("Input C [edit]: ")
                            .top_left_with_margins_on(ids.tab_capacitance, 30.0, 70.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_capacitance)
                            .set(ids.capacitance_input_label, ui);
            */

            match (lref_input_active, lref_eeprom_active) {
                (true, false) => (),
                (false, true) => (),
                (true, true) => lref_input_active = false,
                (false, false) => lref_eeprom_active = true,
            }

            for v in &mut widget::Toggle::new(lref_input_active)
                .top_left_with_margins_on(ids.tab_capacitance, 130.0, 10.0)
                .parent(ids.tab_capacitance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w(30.0)
                .set(ids.capacitance_lref1_toggle, ui)
            {
                let n = v.clone();
                if n {
                    lref_input_active = true;
                    lref_eeprom_active = false;
                } else {
                    lref_input_active = false;
                    lref_eeprom_active = true;
                }
            }

            for v in &mut widget::Toggle::new(lref_eeprom_active)
                .top_left_with_margins_on(ids.tab_capacitance, 165.0, 10.0)
                .parent(ids.tab_capacitance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w(30.0)
                .set(ids.capacitance_lref2_toggle, ui)
            {
                let n = v.clone();
                if n {
                    lref_input_active = false;
                    lref_eeprom_active = true;
                } else {
                    lref_input_active = true;
                    lref_eeprom_active = false;
                }
            }

            widget::Text::new("Input L [ mesured ]: ")
                .top_left_with_margins_on(ids.tab_capacitance, 130.0, 60.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_input_l_label, ui);

            widget::Text::new("Saved L [EEPROM]: ")
                .top_left_with_margins_on(ids.tab_capacitance, 165.0, 60.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_eeprom_l_label, ui);

            if let Some((c0, l0, n)) = eeprom.c0_show() {
                widget::Text::new(&format!("C0: {:.2} pF, L0: {:.2} µH ({:?})", c0, l0, n))
                    .top_left_with_margins_on(ids.tab_capacitance, 165.0, 560.0)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .parent(ids.tab_capacitance)
                    .set(ids.capacitance_eeprom_lc_label, ui);
            } else {
                widget::Text::new("C0: None pF, L0: None µH")
                    .top_left_with_margins_on(ids.tab_capacitance, 165.0, 560.0)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .parent(ids.tab_capacitance)
                    .set(ids.capacitance_eeprom_lc_label, ui);
            }

            f1_f2.new_tab(
                Box::new(ActionButton {
                    parent_id: ids.tab_capacitance,
                    button_id: ids.capacitance_measure_button,
                    enabled: flcq.is_init(),
                }),
                ids.capacitance_f1_label,
                ids.capacitance_current_temperature_f1_label,
                ids.capacitance_calibration_temperature_f1_label,
                ids.capacitance_delta_temperature_f1_label,
                ids.capacitance_f2_label,
                ids.capacitance_current_temperature_f2_label,
                ids.capacitance_calibration_temperature_f2_label,
                ids.capacitance_delta_temperature_f2_label,
            );
            match f1_f2.show(ui) {
                EClick::FNONE => (),
                EClick::F1 => f1_f2.f1_set(frequency_pack(&mut flcq)),
                EClick::F2 => f1_f2.f2_set(frequency_pack(&mut flcq)),
                EClick::END(f1, f2) => {
                    if let (Some(c1), Some(c2)) = cref_source.crefs() {
                        if let Some(c0) = eeprom.c0() {
                            let c = calc_c(f1, f2, c1, c2, c0);

                            widget::Text::new(&format!("C: {:.2} pF", c))
                                .bottom_left_with_margins_on(ids.tab_capacitance, 120.0, 20.0)
                                .color(conrod::color::BLACK)
                                .font_size(35)
                                .line_spacing(3.0)
                                .parent(ids.tab_capacitance)
                                .set(ids.capacitance_c_label, ui);

                            if widget::Button::new()
                                .w_h(220.0, 70.0)
                                .bottom_left_with_margins_on(ids.tab_capacitance, 100.0, 300.0)
                                .label("SAVE as Cref1")
                                .label_font_size(28)
                                .color(conrod::color::LIGHT_BLUE)
                                .set(ids.capacitance_save_c1_button, ui)
                                .was_clicked()
                            {
                                history.append(Box::new(commands::TSaveCref1 { value: c }));
                            }

                            if widget::Button::new()
                                .w_h(220.0, 70.0)
                                .bottom_left_with_margins_on(ids.tab_capacitance, 100.0, 550.0)
                                .label("SAVE as Cref2")
                                .label_font_size(28)
                                .color(conrod::color::LIGHT_BLUE)
                                .set(ids.capacitance_save_c2_button, ui)
                                .was_clicked()
                            {
                                history.append(Box::new(commands::TSaveCref2 { value: c }));
                            }
                        }
                    }
                    history.eeprom(&mut (*eeprom));
                }
            }

            /*
                        const WIDTH_PORTS: conrod::Scalar = 100.0f64;
                        let (mut events, scrollbar) = widget::ListSelect::single(list.len())
                            .flow_down()
                            .item_size(60.0)
                            .scrollbar_next_to()
                            .top_left_with_margins_on(ids.settings, 0.0, 0.0)
                            .w(WIDTH_PORTS)
                            .set(ids.ports, ui);

                        while let Some(event) = events.next(ui, |_i| {
                            println!("_i: {:?}", _i);
                            Some(_i) == selected
                        }) {
                            use conrod::widget::list_select::Event;
                            match event {
                                // For the `Item` events we instantiate the `List`'s items.
                                Event::Item(item) => {
                                    let label = &list[item.i].port_name;

                                    let button = widget::Button::new()
                                        .color(conrod::color::LIGHT_BLUE)
                                        .label(label)
                                        .label_font_size(30)
                                        .label_color(conrod::color::YELLOW);
                                    item.set(button, ui);
                                }

                                // The selection has changed.
                                Event::Selection(selection) => {
                                    //selection.update_index_set(&mut list_selected);
                                    println!("selected indices: {:?}", selection);
                                }

                                // The remaining events indicate interactions with the `ListSelect` widget.
                                _event => {
                                    ()
                                    //println!("{:?}", &event),
                                }
                            }
                        }

                        // Instantiate the scrollbar for the list.
                        if let Some(s) = scrollbar {
                            s.set(ui);
                        }
            */

            if flcq.is_init() {
                conrod::widget::Circle::fill(25.0)
                    .bottom_right_with_margins_on(ids.uart, 5.0, 5.0)
                    .color(conrod::color::GREEN)
                    .set(ids.uart_led, ui);
            } else {
                conrod::widget::Circle::fill(25.0)
                    .bottom_right_with_margins_on(ids.uart, 5.0, 5.0)
                    .color(conrod::color::RED) //conrod::color::RED
                    .set(ids.uart_led, ui);
            }

            /*
                        fn text(text: widget::Text) -> widget::Text {
                            text.color(color::BLACK).font_size(36)
                        }
            */
            /*
            let frequency = f.to_string() + "Hz";
            widget::Text::new(&frequency)
                //.padded_w_of(ids.left_col, PAD)
                .mid_top_with_margin_on(ids.left_col, PAD)
                .color(conrod::color::BLACK)
                .font_size(22)
                .left_justify()
                .line_spacing(10.0)
                .set(ids.left_text, ui);

            widget::Text::new(&s)
                .mid_top_with_margin_on(ids.middle_col, PAD)
                .color(conrod::color::BLACK)
                .font_size(22)
                .set(ids.middle_text, ui);

            let temperature = flcq.t().to_string() + "C";
            conrod::widget::Text::new(&temperature)
                .mid_top_with_margin_on(ids.right_col, PAD)
                .color(conrod::color::BLACK)
                .font_size(22)
                .set(ids.right_text, ui);

            for _click in conrod::widget::Button::new()
                .middle_of(ids.left_col)
                .set(ids.refresh, ui)
            {
                ();
            }*/
        }

        display
            .gl_window()
            .window()
            .set_cursor(conrod::backend::winit::convert_mouse_cursor(
                ui.mouse_cursor(),
            ));

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn frequency_pack(com_port: &mut com::Flcq) -> FpackT {
    let c = com_port.eeprom_read_byte(&0u8);
    let r: FpackT; // read N count
    match com_port.get_frequency_c(&c) {
        (Some(f), _) => {
            r = (
                Some((
                    f,
                    com_port.eeprom_read_f64(&9u8),
                    (com_port.t(), com_port.eeprom_read_f64(&17u8)),
                )),
                "".to_string(),
            )
        }

        (None, str) => r = (None, str),
    };
    r
}

fn temperature(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    tab: conrod::widget::id::Id,
    id: conrod::widget::id::Id,
    t: (f64, f64, f64),
) {
    let (t, tc, d) = t;
    widget::Text::new(&format!(
        "current temperature:       {:.2} C,\nfrequency calibration temperature: {:.2} C,\ndifference: {:.2} C",
        t,
        tc,
        d
    )).bottom_left_with_margins_on(tab, 130.0, 20.0)
        .color(conrod::color::BLACK)
        .font_size(35)
        .line_spacing(4.0)
        .set(id, ui);
    /*,
        (None, str) => widget::Text::new(&str)
            .color(conrod::color::BLACK)
            .top_left_with_margins_on(ids.error, 5.0, 5.0)
            .right_justify()
            .font_size(16)
            .line_spacing(3.0)
            .set(ids.error_label, ui),
    }*/
}

fn swap_f(f1: f64, f2: f64) -> bool {
    let r;
    if f1 < f2 {
        r = false;
    } else {
        r = true;
    }
    r
}

fn swap_c(c1: f64, c2: f64) -> bool {
    swap_f(c2, c1)
}

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

fn calc_c(f1: f64, f2: f64, c1: f64, c2: f64, c0: f64) -> f64 {
    let f1_2 = f1 * f1;
    let f2_2 = f2 * f2;

    (f1_2 * c1 - f2_2 * c2) / (f2_2 - f1_2) - c0
}

fn freq_show(ui: &mut conrod::UiCell, ids: &Ids, text: String) -> Option<f64> {
    let mut res = None;

    for edit in &widget::TextEdit::new(&text)
        .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0 + 100.0, 380.0)
        .color(color::BLACK)
        .font_size(45)
        .line_spacing(2.0)
        .w(500.0)
        .wrap_by_character()
        .center_justify()
        .restrict_to_height(false) // Let the height grow infinitely and scroll.
        .parent(ids.tab_frequency_calibration)
        .set(ids.ref_frequency, ui)
    {
        let s = edit.clone();
        let f = s.parse::<f64>().unwrap();
        res = Some(f);
    }
    res
}

fn edit_ref_frequency(ui: &mut conrod::UiCell, ids: &Ids, freq: f64) -> f64 {
    widget::Text::new(&"Reference frequency: ".to_string())
        .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0 + 100.0, 20.0)
        .color(conrod::color::BLACK)
        .font_size(45)
        .line_spacing(1.0)
        .set(ids.ref_frequency_1, ui);

    if freq < 1000.0 {
        mhz_lebel(ui, ids, "Hz".to_string());
        let r = freq_show(ui, ids, format!("{:.2}", freq));
        match r {
            Some(hz) => hz,
            None => freq,
        }
    } else if 1000.0 < freq && freq < 1000_000.0 {
        mhz_lebel(ui, ids, "kHz".to_string());
        let r = freq_show(ui, ids, format!("{:.5}", freq / 1000.0));
        match r {
            Some(k_hz) => k_hz * 1000.0,
            None => freq,
        }
    } else {
        mhz_lebel(ui, ids, "MHz".to_string());
        let r = freq_show(ui, ids, format!("{:.8}", freq / 1000_000.0));
        match r {
            Some(m_hz) => m_hz * 1000_000.0,
            None => freq,
        }
    }
}

fn mhz_lebel(ui: &mut conrod::UiCell, ids: &Ids, text: String) {
    widget::Text::new(&text)
        .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0 + 100.0, 800.0)
        .color(conrod::color::BLACK)
        .font_size(45)
        .line_spacing(1.0)
        .set(ids.ref_frequency_2, ui);
}

fn error(ui: &mut conrod::UiCell, ids: &Ids, error: &std::string::String) {
    widget::Text::new(error)
        .color(conrod::color::BLACK)
        .top_left_with_margins_on(ids.error, 5.0, 5.0)
        .right_justify()
        .font_size(16)
        .line_spacing(3.0)
        .set(ids.error_label, ui)
}

impl TClicked for ActionButton {
    fn clicked(&mut self, button_label: &str, ui: &mut conrod::UiCell) -> bool {
        widget::Button::new()
            .w_h(220.0, 70.0)
            .bottom_left_with_margins_on(self.parent_id, 100.0, 780.0)
            .label_font_size(45)
            .enabled(self.enabled)
            .label(button_label)
            .parent(self.parent_id)
            .set(self.button_id, ui)
            .was_clicked()
    }

    fn parent_id(&mut self) -> widget::Id {
        self.parent_id
    }
}

impl BToggle {
    pub fn set_ids(
        &mut self,
        parent_id: widget::Id,
        input_id: widget::Id,
        eeprom_id: widget::Id,
        input_label_id: widget::Id,
        eeprom_label_id: widget::Id,
    ) -> () {
        self.parent_id = parent_id;
        self.input_id = input_id;
        self.eeprom_id = eeprom_id;
        self.input_label_id = input_label_id;
        self.eeprom_label_id = eeprom_label_id;
        self.inited_ids = true;
    }
}

impl BToggle {
    pub fn update(&mut self, ui: &mut conrod::UiCell) -> () {
        if self.inited_ids {
            match (self.cref_input_active, self.cref_eeprom_active) {
                (true, false) => (),
                (false, true) => (),
                (true, true) => self.cref_input_active = false,
                (false, false) => self.cref_eeprom_active = true,
            }

            for v in &mut widget::Toggle::new(self.cref_input_active)
                .top_left_with_margins_on(self.parent_id, 30.0, 10.0)
                .parent(self.parent_id)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w_h(30.0, 30.0)
                .set(self.input_id, ui)
            {
                //let n = v.clone();
                let n = v.clone();
                if n {
                    self.cref_input_active = true;
                    self.cref_eeprom_active = false;
                } else {
                    self.cref_input_active = false;
                    self.cref_eeprom_active = true;
                }
            }

            widget::Text::new("Input C [edit]: ")
                .top_left_with_margins_on(self.parent_id, 30.0, 50.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(self.parent_id)
                .set(self.input_label_id, ui);

            for v in &mut widget::Toggle::new(self.cref_eeprom_active)
                .top_left_with_margins_on(self.parent_id, 65.0, 10.0)
                .parent(self.parent_id)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w_h(30.0, 30.0)
                .set(self.eeprom_id, ui)
            {
                let n = v.clone();
                if n {
                    self.cref_input_active = false;
                    self.cref_eeprom_active = true;
                } else {
                    self.cref_input_active = true;
                    self.cref_eeprom_active = false;
                }
            }

            widget::Text::new("Saved C [EEPROM]: ")
                .top_left_with_margins_on(self.parent_id, 65.0, 50.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(self.parent_id)
                .set(self.eeprom_label_id, ui);
        }
    }

    pub fn get(&mut self) -> (bool, bool) {
        (self.cref_input_active, self.cref_eeprom_active)
    }
    pub fn eeprom_undefined(&mut self) -> () {
        self.cref_eeprom_active = false;
        self.cref_input_active = true;
    }
}

impl InputCRef {
    fn update_capacitance_tab_cref(&mut self, eeprom: &Eeprom, ui: &mut conrod::UiCell) {
        let (
            parent_id,
            input_toggle,
            eeprom_toggle,
            input_label,
            eeprom_label,
            cref1_label,
            cref1_text,
            cref1_pf,
            cref2_label,
            cref2_text,
            cref2_pf,
            e_cref1_label,
            e_cref1_text,
            e_cref1_pf,
            e_cref2_label,
            e_cref2_text,
            e_cref2_pf,
        ) = self.tab_id[&"capacitance".to_string()];

        self.update(
            ui,
            eeprom,
            parent_id,
            input_toggle,
            eeprom_toggle,
            input_label,
            eeprom_label,
            cref1_label,
            cref1_text,
            cref1_pf,
            cref2_label,
            cref2_text,
            cref2_pf,
            e_cref1_label,
            e_cref1_text,
            e_cref1_pf,
            e_cref2_label,
            e_cref2_text,
            e_cref2_pf,
        );
    }
}

impl InputCRef {
    fn update_inductance_tab_cref(&mut self, eeprom: &Eeprom, ui: &mut conrod::UiCell) {
        let (
            parent_id,
            input_toggle,
            eeprom_toggle,
            input_label,
            eeprom_label,
            cref1_label,
            cref1_text,
            cref1_pf,
            cref2_label,
            cref2_text,
            cref2_pf,
            e_cref1_label,
            e_cref1_text,
            e_cref1_pf,
            e_cref2_label,
            e_cref2_text,
            e_cref2_pf,
        ) = self.tab_id[&"inductance".to_string()];

        self.update(
            ui,
            eeprom,
            parent_id,
            input_toggle,
            eeprom_toggle,
            input_label,
            eeprom_label,
            cref1_label,
            cref1_text,
            cref1_pf,
            cref2_label,
            cref2_text,
            cref2_pf,
            e_cref1_label,
            e_cref1_text,
            e_cref1_pf,
            e_cref2_label,
            e_cref2_text,
            e_cref2_pf,
        );
    }
}

impl InputCRef {
    fn init(&mut self, ids: &Ids) {
        self.tab_id.insert(
            "capacitance".to_string(),
            (
                ids.tab_capacitance,
                ids.capacitance_input_toggle,
                ids.capacitance_eeprom_toggle,
                ids.capacitance_input_label,
                ids.capacitance_eeprom_label,
                ids.capacitance_cref1_label,
                ids.capacitance_cref1_edit,
                ids.capacitance_cref1_pf,
                ids.capacitance_cref2_label,
                ids.capacitance_cref2_edit,
                ids.capacitance_cref2_pf,
                ids.capacitance_e_cref1_label,
                ids.capacitance_e_cref1_value,
                ids.capacitance_e_cref1_pf,
                ids.capacitance_e_cref2_label,
                ids.capacitance_e_cref2_value,
                ids.capacitance_e_cref2_pf,
            ),
        );

        self.tab_id.insert(
            "inductance".to_string(),
            (
                ids.tab_inductance,
                ids.inductance_input_toggle,
                ids.inductance_eeprom_toggle,
                ids.inductance_input_label,
                ids.inductance_eeprom_label,
                ids.inductance_cref1_label,
                ids.inductance_cref1_edit,
                ids.inductance_cref1_pf,
                ids.inductance_cref2_label,
                ids.inductance_cref2_edit,
                ids.inductance_cref2_pf,
                ids.inductance_e_cref1_label,
                ids.inductance_e_cref1_value,
                ids.inductance_e_cref1_pf,
                ids.inductance_e_cref2_label,
                ids.inductance_e_cref2_value,
                ids.inductance_e_cref2_pf,
            ),
        );
    }
}

impl InputCRef {
    fn crefs(&mut self) -> (Option<f64>, Option<f64>) {
        let (icref, ecref) = self.toggle.get();

        if let (false, true, Some(_), Some(_)) = (icref, ecref, self.ecref1, self.ecref2) {
            return (self.ecref1, self.ecref2);
        }

        (self.icref1, self.icref2)
    }
}

impl InputCRef {
    fn if_swap_c(cref1: &mut Option<f64>, cref2: &mut Option<f64>) {
        if let (Some(c1), Some(c2)) = (cref1.clone(), cref2.clone()) {
            if swap_c(c1, c2) {
                let a = cref2.clone();
                let b = cref1.clone();
                *cref1 = a;
                *cref2 = b;
            }
        }
    }
}

impl InputCRef {
    fn update(
        &mut self,
        ui: &mut conrod::UiCell,
        eeprom: &Eeprom,
        parent_id: widget::Id,
        input_toggle: widget::Id,
        eeprom_toggle: widget::Id,
        input_label: widget::Id,
        eeprom_label: widget::Id,
        cref1_label: widget::Id,
        cref1_text: widget::Id,
        cref1_pf: widget::Id,
        cref2_label: widget::Id,
        cref2_text: widget::Id,
        cref2_pf: widget::Id,
        e_cref1_label: widget::Id,
        e_cref1_value: widget::Id,
        e_cref1_pf: widget::Id,
        e_cref2_label: widget::Id,
        e_cref2_value: widget::Id,
        e_cref2_pf: widget::Id,
    ) {
        {
            self.toggle.set_ids(
                parent_id,
                input_toggle,
                eeprom_toggle,
                input_label,
                eeprom_label,
            );
            self.toggle.update(ui);

            widget::Text::new("( Cref1: ")
                .top_left_with_margins_on(parent_id, 30.0, 350.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(cref1_label, ui);

            match self.icref1.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("[ {:.2} ]", f))
                        .top_left_with_margins_on(parent_id, 30.0, 100.0 + 100.0 + 458.0 - 300.0)
                        .color(color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(parent_id)
                        .set(cref1_text, ui)
                    {
                        let mut s = edit.clone().trim().to_string();
                        {
                            s.remove(0);
                            let end = s.len() - 1;
                            s.remove(end);
                        }
                        match s.trim().parse::<f64>() {
                            Ok(f) => {
                                if 9.0 < f && f < 10000.99 {
                                    self.icref1 = Some(f);
                                }
                            } // if Ok(255), set x to 255
                            Err(e) => println!("{}", e), // if Err("some message"), panic with error message "some message"
                        }
                    }
                }
                None => self.icref1 = Some(1000.0),
            }

            widget::Text::new("pF  > ")
                .top_left_with_margins_on(parent_id, 30.0, 100.0 + 72.0 + 740.0 - 300.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(cref1_pf, ui);

            widget::Text::new(" Cref2: ")
                .top_left_with_margins_on(parent_id, 30.0, 100.0 + 100.0 + 450.0 + 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(cref2_label, ui);

            match self.icref2.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("[ {:.2} ]", f))
                        .top_left_with_margins_on(parent_id, 30.0, 100.0 + 100.0 + 458.0 + 40.0)
                        .color(color::BLACK)
                        //.background_color(color::LIGHT_YELLOW)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(parent_id)
                        .set(cref2_text, ui)
                    {
                        let mut s = edit.clone().to_string();
                        {
                            s.remove(0);
                            let end = s.len() - 1;
                            s.remove(end);
                        }

                        match s.trim().parse::<f64>() {
                            Ok(f) => {
                                if 9.0 < f && f < 10000.99 {
                                    self.icref2 = Some(f);
                                }
                            } // if Ok(255), set x to 255
                            Err(e) => println!("{}", e), // if Err("some message"), panic with error message "some message"
                        }
                    }
                }
                None => self.icref2 = Some(200.0),
            }

            widget::Text::new("pF )")
                .top_left_with_margins_on(parent_id, 30.0, 100.0 + 72.0 + 740.0 + 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(cref2_pf, ui);

            InputCRef::if_swap_c(&mut self.icref1, &mut self.icref2);

            widget::Text::new("( Cref1: ")
                .top_left_with_margins_on(parent_id, 65.0, 100.0 + 100.0 + 450.0 - 300.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(e_cref1_label, ui);
            {
                let mut cref_str = "None".to_string();
                if let Some((cref, n)) = eeprom.cref1_show() {
                    self.ecref1 = Some(cref);
                    cref_str = format!("{:.2} pF({:?})", cref, n);
                } else {
                    self.toggle.eeprom_undefined();
                }

                widget::Text::new(&cref_str)
                    .top_left_with_margins_on(parent_id, 65.0, 100.0 + 100.0 + 458.0 - 150.0)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .parent(parent_id)
                    .set(e_cref1_value, ui);
            }

            widget::Text::new(" Cref2: ")
                .top_left_with_margins_on(parent_id, 65.0, 100.0 + 100.0 + 450.0 + 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(parent_id)
                .set(e_cref2_label, ui);
            {
                let mut cref_str = "None".to_string();
                if let Some((cref, n)) = eeprom.cref2_show() {
                    self.ecref2 = Some(cref);
                    cref_str = format!("{:.2} pF({:?})", cref, n);
                } else {
                    self.toggle.eeprom_undefined();
                }

                widget::Text::new(&cref_str)
                    .top_left_with_margins_on(parent_id, 65.0, 100.0 + 100.0 + 450.0 + 150.0)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .parent(parent_id)
                    .set(e_cref2_value, ui);
            }

            InputCRef::if_swap_c(&mut self.ecref1, &mut self.ecref2);
        }
    }
}

impl<'a> TwoFreq<'a> {
    fn new(input_ab: Box<dyn TClicked + 'a>, error: widget::Id, error_label: widget::Id) -> Self {
        let d = [widget::Id::default(), widget::Id::default()];
        TwoFreq {
            ab: input_ab,
            f1: FpackT::default(),
            f2: FpackT::default(),
            error: error,
            error_label: error_label,
            f_label: d,
            calibration_temperature_f_label: d,
            current_temperature_f_label: d,
            delta_temperature_f_label: d,
        }
    }
}

impl<'a> TwoFreq<'a> {
    fn new_tab(
        &mut self,
        input_ab: Box<dyn TClicked + 'a>,
        f1_label: widget::Id,
        current_temperature_f1_label: widget::Id,
        calibration_temperature_f1_label: widget::Id,
        delta_temperature_f1_label: widget::Id,
        f2_label: widget::Id,
        current_temperature_f2_label: widget::Id,
        calibration_temperature_f2_label: widget::Id,
        delta_temperature_f2_label: widget::Id,
    ) {
        self.ab = input_ab;
        self.f_label = [f1_label, f2_label];
        self.calibration_temperature_f_label = [
            calibration_temperature_f1_label,
            calibration_temperature_f2_label,
        ];
        self.current_temperature_f_label =
            [current_temperature_f1_label, current_temperature_f2_label];
        self.delta_temperature_f_label = [delta_temperature_f1_label, delta_temperature_f2_label];
    }
}

enum EClick {
    FNONE,
    F1,
    F2,
    END(f64, f64),
}

impl<'a> TwoFreq<'a> {
    fn show(&mut self, ui: &mut conrod::UiCell) -> EClick {
        let mut r = EClick::FNONE;
        if let ((Some(f1_pack), _str1), (Some(f2_pack), _str2)) = (self.f1.clone(), self.f2.clone())
        {
            let (f1_, p1_, _) = f1_pack;
            let (f2_, p2_, _) = f2_pack;
            if swap_f(f1_ / p1_, f2_ / p2_) {
                let a = self.f2.clone();
                let b = self.f1.clone();
                self.f1 = a;
                self.f2 = b;
            }
        }

        match (self.f1.clone(), self.f2.clone()) {
            ((None, str), _) => {
                widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(self.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(self.error_label, ui);

                if self.ab.clicked("F1", ui) {
                    r = EClick::F1;
                }
            }
            ((Some(f1), _), (None, str)) => {
                if self.ab.clicked("F2", ui) {
                    r = EClick::F2;
                }
                self.f1_show(ui, f1);

                widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(self.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(self.error_label, ui);
            }
            ((Some(f1), _), (Some(f2), _)) => {
                self.f1_show(ui, f1.clone());
                self.f2_show(ui, f2.clone());

                if self.ab.clicked("CLEAR", ui) {
                    self.f1 = (None, "".to_string());
                    self.f2 = (None, "".to_string());
                }
                let (f1f, p1f, _) = f1.clone();
                let (f2f, p2f, _) = f2.clone();
                r = EClick::END(f1f / p1f, f2f / p2f);
                //(self.f)()
            }
        }
        r
    }
}

impl<'a> TwoFreq<'a> {
    fn f1_show(
        &mut self,
        ui: &mut conrod::UiCell,
        f1: (f64, f64, ((Option<f64>, std::string::String), f64)),
    ) {
        self.f_show(
            ui,
            f1,
            (230.0, 20.0),
            (260.0, 20.0),
            (340.0 - 50.0, 20.0),
            (370.0 - 50.0, 20.0),
            0usize,
        );
    }

    fn f2_show(
        &mut self,
        ui: &mut conrod::UiCell,
        f2: (f64, f64, ((Option<f64>, std::string::String), f64)),
    ) {
        self.f_show(
            ui,
            f2,
            (230.0, 590.0),
            (260.0, 590.0),
            (340.0 - 50.0, 590.0),
            (370.0 - 50.0, 590.0),
            1usize,
        );
    }
}

impl<'a> TwoFreq<'a> {
    fn f_show(
        &mut self,
        ui: &mut conrod::UiCell,
        f1: (f64, f64, ((Option<f64>, std::string::String), f64)),
        xy_f1: (f64, f64),
        xy_current_t: (f64, f64),
        xy_cal_t: (f64, f64),
        xy_delta_t: (f64, f64),
        i: usize,
    ) {
        let prnt = self.ab.parent_id();
        let (f, p, t) = f1;

        {
            let (x, y) = xy_f1;
            widget::Text::new(&format!("F1: {:.2} Hz", f / p))
                .top_left_with_margins_on(prnt, x, y)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .right_justify()
                .parent(prnt)
                .set(self.f_label[i], ui);
        }

        let (current, t_calibration) = t;
        if let (Some(t_now), _) = current {
            {
                let (x, y) = xy_current_t;
                widget::Text::new(&format!("current temperature: {:.2} C", t_now))
                    .top_left_with_margins_on(prnt, x, y)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .right_justify()
                    .parent(prnt)
                    .set(self.current_temperature_f_label[i], ui);
            }
            {
                let (x, y) = xy_cal_t;
                widget::Text::new(&format!("calibration temperature: {:.2} C", t_calibration))
                    .top_left_with_margins_on(prnt, x, y)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .right_justify()
                    .parent(prnt)
                    .set(self.calibration_temperature_f_label[i], ui);
            }
            {
                let (x, y) = xy_delta_t;
                widget::Text::new(&format!("delta: {:.2} C", t_now - t_calibration))
                    .top_left_with_margins_on(prnt, x, y)
                    .color(conrod::color::BLACK)
                    .font_size(25)
                    .line_spacing(3.0)
                    .right_justify()
                    .parent(prnt)
                    .set(self.delta_temperature_f_label[i], ui);
            }
        }
    }
}

impl<'a> TwoFreq<'a> {
    fn f1_set(&mut self, f1: FpackT) {
        self.f1 = f1;
    }
}

impl<'a> TwoFreq<'a> {
    fn f2_set(&mut self, f2: FpackT) {
        self.f2 = f2;
    }
}

impl Default for ActionButton {
    fn default() -> Self {
        Self {
            parent_id: widget::Id::default(),
            button_id: widget::Id::default(),
            enabled: false,
        }
    }
}

impl Default for InputCRef {
    fn default() -> Self {
        Self {
            toggle: BToggle::default(),
            icref1: None,
            icref2: None,
            ecref1: None,
            ecref2: None,
            tab_id: std::collections::HashMap::default(),
        }
    }
}

impl Default for BToggle {
    fn default() -> Self {
        Self {
            parent_id: widget::Id::default(),
            input_id: widget::Id::default(),
            eeprom_id: widget::Id::default(),
            input_label_id: widget::Id::default(),
            eeprom_label_id: widget::Id::default(),
            cref_input_active: true,
            cref_eeprom_active: false,
            inited_ids: false,
        }
    }
}
