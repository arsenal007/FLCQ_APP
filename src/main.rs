extern crate clap;
extern crate dirs;
extern crate serialport;

//use std::io::{self, Write};

#[macro_use]
extern crate conrod;

use clap::{App, AppSettings, Arg};
use conrod::backend::glium::glium::{self, Surface};
use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
mod com;

conrod::widget_ids! {
    struct Ids {
        master,
        middle_col,
        right_col,
        left_text,
        middle_text,
        right_text,
        text,
        refresh,
        tab_frequency,
        tab_frequency_calibration,
        tab_capacity,
        label_frequency,
        label_frequency_calibration,
        label_capacity,
        tabs,
        ports,
        settings,
        top,
        led1,
        label_port,
        connect_button,
        count_frequency_slider,
        count_label,
        count_label_info,
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

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 400;

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
        .insert_from_file("C:\\Users\\Vasyl\\Downloads\\Noto-hinted\\NotoSans-Regular.ttf")
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
                            .length(678.0)
                            .pad(0.0),
                    ),
                    (
                        ids.settings,
                        widget::Canvas::new()
                            .color(conrod::color::WHITE)
                            .length(70.0)
                            .pad(0.0),
                    ),
                ])
                .set(ids.master, ui);

            conrod::widget::Tabs::new(&[
                (ids.tab_frequency, "FREQUENCY"),
                (ids.tab_frequency_calibration, "frequency calibration"),
                (ids.tab_capacity, "capacity measurments"),
            ])
            .parent(ids.top)
            .middle()
            .layout_horizontally()
            .color(conrod::color::WHITE)
            .label_color(conrod::color::BLACK)
            .starting_canvas(ids.tab_frequency)
            .label_font_size(38)
            .set(ids.tabs, ui);

            const MARGIN: conrod::Scalar = 0.0;

            widget::Text::new("UART PORT: ")
                //.padded_w_of(ids.left_col, PAD)
                .mid_left_with_margin_on(ids.settings, MARGIN)
                .color(conrod::color::BLACK)
                .font_size(38)
                .line_spacing(0.0)
                .set(ids.label_port, ui);

            const WIDTH_PORT: conrod::Scalar = 40.0;

            let ports = widget::DropDownList::new(&a, selected_uart_port)
                .scrollbar_next_to()
                .max_visible_items(1usize)
                .h_of(ids.settings)
                .w(300.0)
                .scrollbar_width(WIDTH_PORT)
                .color(color::YELLOW)
                .label_font_size(38)
                .center_justify_label()
                .top_left_with_margins_on(ids.settings, 0.0, 300.0)
                .set(ids.ports, ui);

            match ports {
                Some(id) => {
                    println!("id {}\n", id);
                    selected_uart_port = Some(id)
                }
                None => (),
            }

            if flcq.is_init() {
                let button = "Click to Disconnect";
                if widget::Button::new()
                    .top_left_with_margins_on(ids.settings, 0.0, 600.0)
                    .h_of(ids.settings)
                    .w(350.0)
                    .label(button)
                    .label_font_size(38)
                    .color(conrod::color::GREEN)
                    .set(ids.connect_button, ui)
                    .was_clicked()
                {
                    flcq.disconnect();
                }
            } else {
                let button = "Click to Connect";
                if widget::Button::new()
                    .top_left_with_margins_on(ids.settings, 0.0, 600.0)
                    .h_of(ids.settings)
                    .w(350.0)
                    .label(button)
                    .label_font_size(38)
                    .color(conrod::color::RED)
                    .set(ids.connect_button, ui)
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

            const PAD: conrod::Scalar = 100.0;

            let (min, max) = frequency_count_intervals;

            for value in widget::Slider::new(count as f64, min, max)
                .color(color::LIGHT_BLUE)
                .w_of(ids.tab_frequency_calibration)
                .h(40.0)
                .mid_bottom_with_margin_on(ids.tab_frequency_calibration, PAD)
                .set(ids.count_frequency_slider, ui)
            {
                println!("start {}", value);

                let value: f64 = value;
                count = value.round() as u8;
            }

            const TICKS_BOTTOM_PAD: conrod::Scalar = 155.0;
            const TICKS_INFO_LEFT_PAD: conrod::Scalar = 30.0;
            let text = "Period ticks: ".to_string();

            widget::Text::new(&text)
                .bottom_left_with_margins_on(
                    ids.tab_frequency_calibration,
                    TICKS_BOTTOM_PAD,
                    TICKS_INFO_LEFT_PAD,
                )
                .color(conrod::color::BLACK)
                .font_size(38)
                .line_spacing(1.0)
                .set(ids.count_label_info, ui);

            let text = format!("{:>5}", count);
            let text = text + " [aprox. ";
            let pp = format!("{:.5}", (count as f64) * 0.1048576);
            let text = text + &pp;
            let text = text + " Sec ]";

            const TICKS_LEFT_PAD: conrod::Scalar = 250.0;

            widget::Text::new(&text)
                .bottom_left_with_margins_on(
                    ids.tab_frequency_calibration,
                    TICKS_BOTTOM_PAD,
                    TICKS_LEFT_PAD,
                )
                .color(conrod::color::BLACK)
                .font_size(38)
                .line_spacing(1.0)
                .set(ids.count_label, ui);

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
                let color = conrod::color::GREEN;
                conrod::widget::Circle::fill(30.0)
                    .bottom_right_with_margins_on(ids.settings, 5.0, 5.0)
                    .color(color)
                    .set(ids.led1, ui);
            } else {
                let color = conrod::color::RED;
                conrod::widget::Circle::fill(30.0)
                    .bottom_right_with_margins_on(ids.settings, 5.0, 5.0)
                    .color(color)
                    .set(ids.led1, ui);
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

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {}
