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
        capacitance_cref1_toggle,
        capacitance_cref2_edit,
        capacitance_cref2_label,
        capacitance_cref2_pf,
        capacitance_cref2_toggle,
        capacitance_input_label,
        capacitance_eeprom_label,
        capacitance_input_l_label,
        capacitance_eeprom_l_label,
        capacitance_lref1_toggle,
        capacitance_lref2_toggle,
        capacitance_eeprom_lc_label,
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
        inductance_cref1_edit,
        inductance_cref1_label,
        inductance_cref1_pf,
        inductance_cref1_toggle,
        inductance_cref2_edit,
        inductance_cref2_label,
        inductance_cref2_pf,
        inductance_cref2_toggle,
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
        inductance_frequency_calibration_temperature,
        inductance_frequency_calibration_temperature_f1_label,
        inductance_frequency_delta_temperature_f1_label,
        inductance_current_temperature_f2_label,
        inductance_frequency_calibration_temperature_f2_label,
        inductance_frequency_delta_temperature_f2_label,

    }
}

type FpackT = (
    Option<(f64, f64, ((Option<f64>, std::string::String), f64))>,
    std::string::String,
);

fn main() {
    let mut flcq: com::Flcq = com::init();
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
        .insert_from_file("C:\\Windows\\Fonts\\times.ttf")
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
    let mut c_ref1 = Some(1000.0);
    let mut c_ref2 = Some(1000.0);
    let mut cref_input_active = true;
    let mut cref_eeprom_active = false;
    let mut lref_input_active = true;
    let mut lref_eeprom_active = false;
    let mut fc = (254u8, (None, "".to_string()), (None, "".to_string()));
    let mut frequency = (None, "".to_string());
    let mut frequency1_l = (None, "".to_string());
    let mut frequency2_l = (None, "".to_string());
    let mut eeprom_lc = None;

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
            widget::Text::new("Cref1: ")
                .top_left_with_margins_on(ids.tab_inductance, 70.0, 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref1_label, ui);

            match c_ref1.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_inductance, 70.0, 50.0)
                        .color(color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_cref1_edit, ui)
                    {
                        let s = edit.clone();
                        let f = s.parse::<f64>().unwrap();
                        if 9.0 < f && f < 10000.99 {
                            c_ref1 = Some(f);
                        }
                    }
                }
                None => c_ref1 = Some(1000.0),
            }

            widget::Text::new("pF")
                .top_left_with_margins_on(ids.tab_inductance, 70.0, 310.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref1_pf, ui);

            widget::Text::new("Cref2: ")
                .top_left_with_margins_on(ids.tab_inductance, 100.0, 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref2_label, ui);

            match c_ref2.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_inductance, 100.0, 50.0)
                        .color(color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_cref2_edit, ui)
                    {
                        let s = edit.clone();
                        let f = s.parse::<f64>().unwrap();
                        if 9.0 < f && f < 10000.99 {
                            c_ref2 = Some(f);
                        }
                    }
                }
                None => c_ref2 = Some(1000.0),
            }

            widget::Text::new("pF")
                .top_left_with_margins_on(ids.tab_inductance, 100.0, 310.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref2_pf, ui);

            match (cref_input_active, cref_eeprom_active) {
                (true, false) => (),
                (false, true) => (),
                (true, true) => cref_input_active = false,
                (false, false) => cref_eeprom_active = true,
            }

            for v in &mut widget::Toggle::new(cref_input_active)
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 10.0)
                .parent(ids.tab_inductance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w(30.0)
                .set(ids.inductance_cref1_toggle, ui)
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
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 550.0)
                .parent(ids.tab_inductance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .w(30.0)
                .set(ids.inductance_cref2_toggle, ui)
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
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 70.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_input_label, ui);

            widget::Text::new("Saved C [EEPROM]: ")
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 610.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_eeprom_label, ui);

            match (frequency1_l.clone(), frequency2_l.clone()) {
                ((None, str), _) => {
                    if widget::Button::new()
                        .w_h(250.0 * 0.8, 100.0 * 0.8)
                        .bottom_left_with_margins_on(ids.tab_inductance, 150.0, 800.0)
                        .label_font_size(45)
                        .enabled(flcq.is_init())
                        .label("F1")
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_measure_button, ui)
                        .was_clicked()
                    {
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
                    if widget::Button::new()
                        .w_h(250.0 * 0.8, 100.0 * 0.8)
                        .bottom_left_with_margins_on(ids.tab_inductance, 150.0, 800.0)
                        .label_font_size(50)
                        .enabled(flcq.is_init())
                        .label("F2")
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_measure_button, ui)
                        .was_clicked()
                    {
                        if flcq.is_init() {
                            frequency2_l = frequency_pack(&mut flcq);
                        }
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

                ((Some(f1), str1), (Some(f2), str2)) => {
                    match (cref_input_active, cref_eeprom_active) {
                        (true, false) => {
                            if let (Some(c1), Some(c2)) = (c_ref1.clone(), c_ref2.clone()) {
                                let (f1_, p1_, t1) = f1;
                                let (f2_, p2_, t2) = f2;

                                let f1__;
                                let f2__;

                                if f1_ < f2_ {
                                    f1__ = f1_ / p1_;
                                    f2__ = f2_ / p2_;
                                } else {
                                    f1__ = f2_ / p2_;
                                    f2__ = f1_ / p1_;
                                    frequency1_l = (Some((f2_, p2_, t1.clone())), str1);
                                    frequency2_l = (Some((f1_, p1_, t2.clone())), str2);
                                }

                                let c1__;
                                let c2__;

                                if c1 > c2 {
                                    c1__ = c1;
                                    c2__ = c2;
                                } else {
                                    c1__ = c2;
                                    c2__ = c1; // in pico farad
                                }

                                let (c, l) = calc_l(f1__, f2__, c1__, c2__);

                                widget::Text::new(&format!("F1: {:.2} Hz", f1__))
                                    .top_left_with_margins_on(ids.tab_inductance, 150.0, 20.0)
                                    .color(conrod::color::BLACK)
                                    .font_size(25)
                                    .line_spacing(3.0)
                                    .parent(ids.tab_inductance)
                                    .set(ids.inductance_f1_label, ui);

                                let (current1, calibration_temperature1) = t1;
                                if let (Some(t98), _) = current1 {
                                    widget::Text::new(&format!(
                                        "current temperature: {:.2} C",
                                        t98
                                    ))
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

                                widget::Text::new(&format!("F2: {:.2} Hz", f2__))
                                    .top_left_with_margins_on(ids.tab_inductance, 150.0, 560.0)
                                    .color(conrod::color::BLACK)
                                    .font_size(25)
                                    .line_spacing(3.0)
                                    .parent(ids.tab_inductance)
                                    .set(ids.inductance_f2_label, ui);

                                let (current2, calibration_temperature2) = t2;
                                if let (Some(t99), _) = current2 {
                                    widget::Text::new(&format!(
                                        "current temperature: {:.2} C",
                                        t99
                                    ))
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

                                if widget::Button::new()
                                    .top_left_with_margins_on(ids.tab_inductance, 420.0, 70.0)
                                    .h(30.0)
                                    .w(350.0)
                                    .label("RESET")
                                    .label_font_size(25)
                                    .color(conrod::color::LIGHT_RED)
                                    .set(ids.inductance_reset_button, ui)
                                    .was_clicked()
                                {
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
                        (false, true) => (),
                        (true, true) => (),
                        (false, false) => (),
                    };
                }
            }
            // ====================================================================================
            // tab C
            // ====================================================================================
            widget::Text::new("Cref1: ")
                .top_left_with_margins_on(ids.tab_capacitance, 70.0, 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_cref1_label, ui);

            match c_ref1.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_capacitance, 70.0, 50.0)
                        .color(color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(ids.tab_capacitance)
                        .set(ids.capacitance_cref1_edit, ui)
                    {
                        let s = edit.clone();
                        let f = s.parse::<f64>().unwrap();
                        if 9.0 < f && f < 10000.99 {
                            c_ref1 = Some(f);
                        }
                    }
                }
                None => c_ref1 = Some(1000.0),
            }

            widget::Text::new("pF")
                .top_left_with_margins_on(ids.tab_capacitance, 70.0, 310.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_cref1_pf, ui);

            widget::Text::new("Cref2: ")
                .top_left_with_margins_on(ids.tab_capacitance, 100.0, 40.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_cref2_label, ui);

            match c_ref2.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_capacitance, 100.0, 50.0)
                        .color(color::BLACK)
                        .font_size(25)
                        .line_spacing(3.0)
                        .w(250.0)
                        .wrap_by_character()
                        .right_justify()
                        .restrict_to_height(false) // Let the height grow infinitely and scroll.
                        .parent(ids.tab_capacitance)
                        .set(ids.capacitance_cref2_edit, ui)
                    {
                        let s = edit.clone();
                        let f = s.parse::<f64>().unwrap();
                        if 9.0 < f && f < 10000.99 {
                            c_ref2 = Some(f);
                        }
                    }
                }
                None => c_ref2 = Some(1000.0),
            }

            widget::Text::new("pF")
                .top_left_with_margins_on(ids.tab_capacitance, 100.0, 310.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_cref2_pf, ui);

            match (cref_input_active, cref_eeprom_active) {
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

            widget::Text::new("Saved C [EEPROM]: ")
                .top_left_with_margins_on(ids.tab_capacitance, 30.0, 610.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_eeprom_label, ui);

            match (lref_input_active, lref_eeprom_active) {
                (true, false) => (),
                (false, true) => (),
                (true, true) => lref_input_active = false,
                (false, false) => lref_eeprom_active = true,
            }

            for v in &mut widget::Toggle::new(lref_input_active)
                .top_left_with_margins_on(ids.tab_capacitance, 140.0, 10.0)
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
                .top_left_with_margins_on(ids.tab_capacitance, 140.0, 550.0)
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

            widget::Text::new("Input L ")
                .top_left_with_margins_on(ids.tab_capacitance, 140.0, 70.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_input_l_label, ui);

            widget::Text::new("Saved L [EEPROM]: ")
                .top_left_with_margins_on(ids.tab_capacitance, 140.0, 610.0)
                .color(conrod::color::BLACK)
                .font_size(25)
                .line_spacing(3.0)
                .parent(ids.tab_capacitance)
                .set(ids.capacitance_eeprom_l_label, ui);

            match eeprom_lc.clone() {
                Some(p) => {
                    let (c, l) = p;
                    if (0.0 < c) && (c < 200.0) && (0.0 < l) && (l < 300.0) {
                        widget::Text::new(&format!("C: {:.2} pF, L {:.2} µH", c, l))
                            .top_left_with_margins_on(ids.tab_capacitance, 180.0, 560.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_capacitance)
                            .set(ids.capacitance_eeprom_lc_label, ui);
                    } else {
                        widget::Text::new("C: NaN pF, L NaN µH")
                            .top_left_with_margins_on(ids.tab_capacitance, 180.0, 560.0)
                            .color(conrod::color::BLACK)
                            .font_size(25)
                            .line_spacing(3.0)
                            .parent(ids.tab_capacitance)
                            .set(ids.capacitance_eeprom_lc_label, ui);
                    }
                }
                None if flcq.is_init() => {
                    let c = flcq.eeprom_read_f64(&25u8);
                    let l = flcq.eeprom_read_f64(&33u8);
                    let pack = (c, l);
                    eeprom_lc = Some(pack);
                }
                None => (),
            }

            match (frequency1_c.clone(), frequency2_c.clone()) {
                ((None, str), _) => {
                    if widget::Button::new()
                        .w_h(250.0 * 0.8, 100.0 * 0.8)
                        .bottom_left_with_margins_on(ids.tab_inductance, 150.0, 800.0)
                        .label_font_size(45)
                        .enabled(flcq.is_init())
                        .label("F1")
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_measure_button, ui)
                        .was_clicked()
                    {
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

fn calc_l(f1: f64, f2: f64, c1: f64, c2: f64) -> (f64, f64) {
    let f1_2 = f1 * f1;
    let f2_2 = f2 * f2;

    let c1f = c1 / 1000_000_000_000.0; // in farad
    let c2f = c2 / 1000_000_000_000.0;

    let c = (f1_2 * c1f - f2_2 * c2f) / (f2_2 - f1_2);

    let l = (1.0 / f1_2 - 1.0 / f2_2)
        / (4.0 * std::f64::consts::PI * std::f64::consts::PI * (c1f - c2f)); // in Henry

    (c * 1000_000_000_000.0, l * 1000_000.0) // return in pico farads and micro Henrys
}

fn calc_c(f1: f64, f2: f64, c1: f64, c2: f64, c0: f64) -> f64 {
    let f1_2 = f1 * f1;
    let f2_2 = f2 * f2;

    let c1f = c1 / 1000_000_000_000.0; // in farad
    let c2f = c2 / 1000_000_000_000.0;

    let c = (f1_2 * c1f - f2_2 * c2f) / (f2_2 - f1_2) - c0;

    c * 1000_000_000_000.0 // return in pico farads
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
