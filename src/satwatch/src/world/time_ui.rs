use crate::util::input_events::Event;
use crate::world::world_ui::WorldUi;
use chrono::{DateTime, Local, Utc};
use glow::Context;
use imgui::{SliderFlags, Ui};
use legion::World;
use libspace::timebase::Timebase;
use std::time::Duration;

pub struct TimeUi {
    visible: bool,
    timebase: Timebase,
}

impl TimeUi {
    pub fn new() -> Self {
        Self {
            visible: true,
            timebase: Timebase::new(),
        }
    }
}

impl WorldUi for TimeUi {
    fn main_menu(&mut self, ui: &Ui) {
        ui.menu("View", || {
            if ui.menu_item("Time") {
                self.visible = true;
            }
        });
    }

    fn ui(&mut self, gl: &Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        if self.visible {
            ui.window("Time Control")
                .opened(&mut self.visible)
                .build(|| {
                    let utc: DateTime<Utc> = self.timebase.now_utc();
                    let local: DateTime<Local> = self.timebase.now_utc().into();
                    ui.text(format!(
                        "Now Julian (since J2000): {} ({})",
                        self.timebase.now_jd(),
                        self.timebase.now_jd_j2000()
                    ));
                    ui.text(format!("Now   UTC: {}", utc));
                    ui.text(format!("Now Local: {}", local));
                    ui.separator();
                    let mut rt = self.timebase.realtime();
                    ui.checkbox("Realtime", &mut rt);
                    self.timebase.set_realtime(rt);

                    if !rt {
                        let mut accel = self.timebase.acceleration();
                        ui.slider("Time Acceleration", -10e8, 10e8, &mut accel);
                        self.timebase.set_acceleration(accel);
                    }
                });
        }
        Ok(())
    }

    fn handle_input(&mut self, _gl: &Context, _world: &mut World, event: Event) {}

    fn tick(
        &mut self,
        _gl: &Context,
        _world: &mut World,
        timebase: &mut Timebase,
    ) -> Result<(), String> {
        self.timebase.tick(1.0 / 60.0);
        *timebase = self.timebase.clone();
        Ok(())
    }
}
