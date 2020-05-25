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
    }
}

fn main() {
    /*let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .get_matches();
    let mut flcq = com::open(matches.value_of("port").unwrap());*/

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
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 200;

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

    for x in list {
        a.push(x.port_name.clone());
    }

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
                            .length(700.0),
                    ),
                    (
                        ids.settings,
                        widget::Canvas::new().color(conrod::color::WHITE),
                    ),
                ])
                .set(ids.master, ui);

            conrod::widget::Tabs::new(&[
                (ids.tab_frequency, "frequency"),
                (ids.tab_frequency_calibration, "frequency calibration"),
                (ids.tab_capacity, "capacity measurments"),
            ])
            //.wh_of(ids.master)
            .parent(ids.top)
            .middle()
            .layout_horizontally()
            .color(conrod::color::WHITE)
            .label_color(conrod::color::BLACK)
            .starting_canvas(ids.tab_frequency)
            .label_font_size(40)
            .set(ids.tabs, ui);

            const MARGIN: conrod::Scalar = 0.0;

            widget::Text::new("UART PORT: ")
                //.padded_w_of(ids.left_col, PAD)
                .mid_left_with_margin_on(ids.settings, MARGIN)
                .color(conrod::color::BLACK)
                .font_size(40)
                .line_spacing(0.0)
                .set(ids.label_port, ui);

            const WIDTH_PORT: conrod::Scalar = 20.0;

            widget::DropDownList::new(&a, Some(1usize))
                .max_visible_items(1usize)
                .h_of(ids.settings)
                .w(300.0)
                .scrollbar_width(WIDTH_PORT)
                .scrollbar_on_top()
                .color(color::WHITE)
                .label_font_size(40)
                .top_left_with_margins_on(ids.settings, 0.0, 300.0)
                .set(ids.ports, ui);

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
            let r: conrod::Scalar = 50.0;
            conrod::widget::Circle::fill(20.0)
                .parent(ids.settings)
                .bottom_right()
                .color(conrod::color::GREEN)
                .set(ids.led1, ui);
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
