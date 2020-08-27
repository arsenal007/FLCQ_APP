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
        tab_capacity,
        tab_inductance,
        tab_crystal,
        label_frequency,
        label_frequency_calibration,
        label_capacity,
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
        inductance_frequency_calibration_temperature,
        inductance_c,
    }
}

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

    let mut count: u8 = 254u8;
    let mut f_ref = None;
    let mut c_ref1 = Some(1000.0);
    let mut c_ref2 = Some(1000.0);
    let mut cref1_active = true;
    let mut cref2_active = false;
    let mut temperature_cal = (None, "".to_string());
    let mut frequency = (None, "".to_string());
    let mut frequency_cal = (None, "".to_string());
    let mut frequency1_l = (None, "".to_string());
    let mut frequency2_l = (None, "".to_string());
    let mut periode = 1.0;

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
                (ids.tab_capacity, "C MEASURMENTS"),
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

            let (min, max) = frequency_count_intervals;

            for value in widget::Slider::new(count as f64, min, max)
                .color(color::LIGHT_BLUE)
                .h(60.0)
                .mid_bottom_with_margin_on(ids.top, 5.0)
                .w_of(ids.tab_frequency_calibration)
                .parent(ids.tab_frequency_calibration)
                .set(ids.count_frequency_slider, ui)
            {
                //println!("start {}", value);
                let value: f64 = value;
                count = value.round() as u8;
            }

            widget::Text::new("Period ticks: ")
                .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 20.0)
                .color(conrod::color::BLACK)
                .right_justify()
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_frequency_calibration)
                .set(ids.count_label_info, ui);

            widget::Text::new(&format!("{:}", count))
                .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 350.0)
                .color(conrod::color::BLACK)
                .right_justify()
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_frequency_calibration)
                .set(ids.count_label, ui);

            let text = " [aprox. ".to_string();
            let pp = format!("{:.5}", (count as f64) * 0.1048576);
            let text = text + &pp;
            let text = text + " Sec ]";

            widget::Text::new(&text)
                .bottom_left_with_margins_on(ids.count_frequency_slider, 80.0, 480.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_frequency_calibration)
                .set(ids.count_label_approx_in_sec, ui);

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
                    frequency = flcq.get_frequency_c(&count);
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
                    flcq.eeprom_write_byte(&0u8, &count); // save N count
                    match &frequency_cal {
                        (Some(f), _) => {
                            flcq.eeprom_write_f64(&1u8, &f);
                            match f_ref {
                                Some(ref_frequency) => {
                                    let periode = f / ref_frequency;
                                    flcq.eeprom_write_f64(&9u8, &periode);
                                }
                                None => (),
                            }
                        }
                        (None, _) => (),
                    }
                    match &temperature_cal {
                        (Some(t), _) => flcq.eeprom_write_f64(&17u8, &t),
                        (None, _) => (),
                    }
                }
            }

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
                    let c = flcq.eeprom_read_byte(&0u8); // read N count
                    frequency = flcq.get_frequency_c(&c);
                    periode = flcq.eeprom_read_f64(&9u8);
                }
            }

            match &frequency {
                (Some(f), _) => {
                    let frequency = f / periode;
                    widget::Text::new(&format!("FREQ: {:.2} Hz", frequency))
                        .bottom_left_with_margins_on(ids.tab_frequency, 480.0, 20.0)
                        .color(conrod::color::BLACK)
                        .right_justify()
                        .font_size(45)
                        .line_spacing(3.0)
                        .set(ids.label_frequency, ui);
                }
                (None, str) => widget::Text::new(&str)
                    .color(conrod::color::BLACK)
                    .top_left_with_margins_on(ids.error, 5.0, 5.0)
                    .right_justify()
                    .font_size(16)
                    .line_spacing(3.0)
                    .set(ids.error_label, ui),
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

            temperature(
                ui,
                &ids,
                ids.tab_frequency,
                ids.frequency_temperature,
                &mut flcq,
            );

            widget::Text::new("Cref1: ")
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 70.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref1_label, ui);

            match c_ref1.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_inductance, 30.0, 120.0)
                        .color(color::BLACK)
                        .font_size(45)
                        .line_spacing(3.0)
                        .w(300.0)
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
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 430.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref1_pf, ui);

            widget::Text::new("Cref2: ")
                .top_left_with_margins_on(ids.tab_inductance, 100.0, 70.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref2_label, ui);

            match c_ref2.clone() {
                Some(f) => {
                    for edit in &widget::TextEdit::new(&format!("{:.2}", f))
                        .top_left_with_margins_on(ids.tab_inductance, 100.0, 120.0)
                        .color(color::BLACK)
                        .font_size(45)
                        .line_spacing(3.0)
                        .w(300.0)
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
                .top_left_with_margins_on(ids.tab_inductance, 100.0, 430.0)
                .color(conrod::color::BLACK)
                .font_size(45)
                .line_spacing(3.0)
                .parent(ids.tab_inductance)
                .set(ids.inductance_cref2_pf, ui);

            if let (false, false) = (cref1_active, cref2_active) {
                cref1_active = true
            }

            for v in &mut widget::Toggle::new(cref1_active)
                .top_left_with_margins_on(ids.tab_inductance, 30.0, 10.0)
                .parent(ids.tab_inductance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .set(ids.inductance_cref1_toggle, ui)
            {
                cref1_active = v.clone();
            }

            for v in &mut widget::Toggle::new(cref2_active)
                .top_left_with_margins_on(ids.tab_inductance, 100.0, 10.0)
                .parent(ids.tab_inductance)
                .enabled(true)
                .color(conrod::color::GREEN)
                .border(4.0)
                .border_color(conrod::color::RED)
                .set(ids.inductance_cref2_toggle, ui)
            {
                cref2_active = v.clone();
            }

            frequency1_l = (Some(30_440_000.0), "".to_string());
            frequency2_l = (Some(11_717_000.0), "".to_string());

            let pack = (frequency1_l.clone(), frequency2_l.clone());

            match pack {
                ((None, str), _) => {
                    if widget::Button::new()
                        .w_h(250.0, 100.0)
                        .bottom_left_with_margins_on(ids.tab_inductance, 450.0, 750.0)
                        .label_font_size(50)
                        .enabled(flcq.is_init())
                        .label("F1")
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_measure_button, ui)
                        .was_clicked()
                    {
                        if flcq.is_init() {
                            let c = flcq.eeprom_read_byte(&0u8); // read N count
                            frequency1_l = flcq.get_frequency_c(&c);
                        }
                    };

                    widget::Text::new(&str)
                        .color(conrod::color::BLACK)
                        .top_left_with_margins_on(ids.error, 5.0, 5.0)
                        .right_justify()
                        .font_size(16)
                        .line_spacing(3.0)
                        .set(ids.error_label, ui);
                }
                ((Some(_f1), _), (None, str)) => {
                    if widget::Button::new()
                        .w_h(250.0, 100.0)
                        .bottom_left_with_margins_on(ids.tab_inductance, 450.0, 750.0)
                        .label_font_size(50)
                        .enabled(flcq.is_init())
                        .label("F2")
                        .parent(ids.tab_inductance)
                        .set(ids.inductance_measure_button, ui)
                        .was_clicked()
                    {
                        if flcq.is_init() {
                            let c = flcq.eeprom_read_byte(&0u8); // read N count

                            frequency2_l = flcq.get_frequency_c(&c);
                        }
                    };

                    widget::Text::new(&str)
                        .color(conrod::color::BLACK)
                        .top_left_with_margins_on(ids.error, 5.0, 5.0)
                        .right_justify()
                        .font_size(16)
                        .line_spacing(3.0)
                        .set(ids.error_label, ui);
                }

                ((Some(fa), _), (Some(fb), _)) => match (cref1_active, cref2_active) {
                    (true, false) => {
                        if let Some(c_ref) = c_ref1 {
                            let (c, l) = l1(fa, fb, 1.0, c_ref);
                            l_tab(ui, &ids, c, l);
                            temperature(
                                ui,
                                &ids,
                                ids.tab_inductance,
                                ids.inductance_frequency_calibration_temperature,
                                &mut flcq,
                            );
                        }
                    }
                    (false, true) => {
                        if let Some(c_ref) = c_ref2 {
                            let (c, l) = l1(fa, fb, flcq.eeprom_read_f64(&9u8), c_ref);
                            l_tab(ui, &ids, c, l);
                            temperature(
                                ui,
                                &ids,
                                ids.tab_inductance,
                                ids.inductance_frequency_calibration_temperature,
                                &mut flcq,
                            );
                        }
                    }
                    (true, true) => (),
                    (false, false) => (),
                },
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

fn l_tab(ui: &mut conrod::UiCell, ids: &Ids, c: f64, l: f64) {
    let txt = &format!("coil L: {:.2} nH, coil C: {:.2} pF", l, c);
    widget::Text::new(txt)
        .top_left_with_margins_on(ids.tab_inductance, 170.0, 30.0)
        .color(conrod::color::BLACK)
        .font_size(45)
        .line_spacing(1.0)
        .set(ids.inductance_c, ui);
}

fn temperature(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    tab: conrod::widget::id::Id,
    id: conrod::widget::id::Id,
    d: &mut com::Flcq,
) {
    //let frequency_calibration_temperature = d.eeprom_read_f64(&17u8);
    let frequency_calibration_temperature = 25.0;
    //match d.t() {
    match (Some(24.0), "".to_string()) {
        (Some(current_temperature), _) => widget::Text::new(&format!(
        "current temperature:       {:.2} C,\nfrequency calibration temperature: {:.2} C,\ndifference: {:.2} C",
        current_temperature,
        frequency_calibration_temperature,
        current_temperature - frequency_calibration_temperature
    ))
        .bottom_left_with_margins_on(tab, 130.0, 20.0)
        .color(conrod::color::BLACK)
        .font_size(35)
        .line_spacing(4.0)
        .set(id, ui),
        (None, str) => widget::Text::new(&str)
            .color(conrod::color::BLACK)
            .top_left_with_margins_on(ids.error, 5.0, 5.0)
            .right_justify()
            .font_size(16)
            .line_spacing(3.0)
            .set(ids.error_label, ui),
    }
}

fn l1(fa: f64, fb: f64, period: f64, c_ref: f64) -> (f64, f64) {
    let mut f1 = 0.0;
    let mut f2 = 0.0;
    if fa < fb {
        f1 = fb / period;
        f2 = fa / period;
    } else {
        f1 = fa / period;
        f2 = fb / period;
    }
    let c = (f2 * f2) / ((f1 * f1) - (f2 * f2)) * c_ref;

    let c_farad = c / 1000_000_000_000.0;
    let l = 1.0 / (4.0 * std::f64::consts::PI * std::f64::consts::PI * f1 * f1 * c_farad);
    (c, l * 1000_000_000.0)
}

fn l2(f1_: f64, f2_: f64, period: f64, c1_: f64, c2_: f64) -> (f64, f64) {
    let f1;
    let f2;

    if f1_ < f2_ {
        f1 = f1_ / period;
        f2 = f2_ / period;
    } else {
        f1 = f2_ / period;
        f2 = f1_ / period;
    }

    let c1;
    let c2;

    if c1_ > c2_ {
        c1 = c1_;
        c2 = c2_;
    } else {
        c1 = c2_;
        c2 = c1_; // in pico farad
    }

    let f1_2 = f1 * f1;
    let f2_2 = f2 * f2;
    let c = (f1_2 * c1 - f2_2 * c2) / (f2_2 - f1_2);

    let c1f = c1 / 1000_000_000_000.0; // in farad
    let c2f = c2 / 1000_000_000_000.0;

    let l = (1.0 / f1_2 - 1.0 / f2_2)
        / (4.0 * std::f64::consts::PI * std::f64::consts::PI * (c1f - c2f)); // in Henry

    (c, l * 1000_000_000.0) // return in pico farads and micro Henrys
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
