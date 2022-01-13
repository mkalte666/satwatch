use imgui::*;
use legion::*;

use crate::components::{Camera, MaterialComponent, VertexList, WorldTransform};
use crate::util::input_events::Event;
use crate::util::vertex_tools::*;
use crate::world::element_db_ui::SelectionChanges;
use crate::world::view_ui::ViewUi;
use crate::world::world_ui::WorldUi;
use glam::{Quat, Vec3, Vec4};
use libspace::bodies::Planet;
use libspace::coordinate::PlanetaryStateVector;
use libspace::coordinate::*;
use libspace::element_db::ElementDb;
use libspace::element_engine::{ElementEngine, ElementUpdate};
use libspace::timebase::Timebase;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

pub struct WorldControl {
    uis: Vec<Box<dyn WorldUi>>,
    last_tick: Instant,
    timebase: Timebase,
}

impl WorldControl {
    pub fn new(gl: &glow::Context, world: &mut World) -> Result<Self, String> {
        Ok(Self {
            uis: vec![Box::new(ViewUi::new(gl, world)?)],
            last_tick: Instant::now(),
            timebase: Timebase::new(),
        })
    }

    pub fn global_tick(&mut self, gl: &glow::Context, world: &mut World) -> Result<(), String> {
        // tick rate housekeeping
        // with early exit if we dont update
        let tick_duration = Duration::from_secs_f64(1.0 / 60.0);

        if Instant::now() - self.last_tick < tick_duration {
            return Ok(());
        }
        self.last_tick = Instant::now();

        let mut old_timebase = self.timebase.clone();
        self.tick(gl, world, &mut old_timebase)?;
        self.timebase = old_timebase;
        Ok(())
    }
}

impl WorldUi for WorldControl {
    fn main_menu(&mut self, ui: &Ui) {
        for wui in &mut self.uis {
            wui.main_menu(ui);
        }
    }

    fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String> {
        for wui in &mut self.uis {
            wui.ui(gl, world, ui)?;
        }

        Ok(())
    }

    fn handle_input(&mut self, gl: &glow::Context, world: &mut World, event: Event) {
        for wui in &mut self.uis {
            wui.handle_input(gl, world, event);
        }
    }

    fn tick(
        &mut self,
        gl: &glow::Context,
        world: &mut World,
        timebase: &mut Timebase,
    ) -> Result<(), String> {
        for wui in &mut self.uis {
            wui.tick(gl, world, timebase)?;
        }

        Ok(())
    }
}
