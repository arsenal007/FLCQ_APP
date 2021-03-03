use conrod::{Colorable, Positionable, Sizeable, Widget};

struct PWidgets {
    top: conrod::widget::Id,
    tab: conrod::widget::Id,
    slider: conrod::widget::Id,
    frequency_counter: u8,
    frequency_calibration: (
        (Option<f64>, std::string::String),
        (Option<f64>, std::string::String),
    ),
}

impl PWidgets {
    const MIN_FREQUENCY_COUNTER: f64 = 1.0f64; // f64
    const MAX_FREQUENCY_COUNTER: f64 = 254.0f64; // f64

    pub fn plot(&self, ui: &mut conrod::UiCell) {
        for value in conrod::widget::Slider::new(
            self.frequency_counter as f64,
            Self::MIN_FREQUENCY_COUNTER,
            Self::MAX_FREQUENCY_COUNTER,
        )
        .color(conrod::color::LIGHT_BLUE)
        .h(60.0)
        .mid_bottom_with_margin_on(self.top, 5.0)
        .w_of(self.tab)
        .parent(self.tab)
        .set(self.slider, ui)
        {
            // println!("start {}", value);
            // let value: f64 = value;
            self.frequency_counter = value.round() as u8;
        }
    }
}

struct FNotCallibrated {
    top: conrod::widget::Id,
    tabs: conrod::widget::Id,
    tab_frequency_calibration: conrod::widget::Id,
}

struct StateMachine<S> {
    permament_widgets: PWidgets,
    state: S,
}

impl FNotCallibrated {
    pub fn plot(&mut self, ui: &mut conrod::UiCell) {
        conrod::widget::Tabs::new(&[(self.tab_frequency_calibration, "F CALIBRATION")])
            .h_of(self.top)
            .parent(self.top)
            .middle()
            .layout_horizontally()
            .color(conrod::color::WHITE)
            .label_color(conrod::color::BLACK)
            .starting_canvas(self.tab_frequency_calibration)
            .label_font_size(50)
            .set(self.tabs, ui);
    }
    fn is_callibrated(&self) -> bool {
        false
    }
}
