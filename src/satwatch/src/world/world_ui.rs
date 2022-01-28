use crate::util::input_events::Event;
use crate::AppPhase;
use imgui::Ui;
use legion::World;
use libspace::timebase::Timebase;

pub trait WorldUi {
    fn main_menu(&mut self, ui: &Ui);

    fn ui(&mut self, gl: &glow::Context, world: &mut World, ui: &mut Ui) -> Result<(), String>;

    fn handle_input(&mut self, _gl: &glow::Context, _world: &mut World, event: Event);

    fn tick(
        &mut self,
        gl: &glow::Context,
        world: &mut World,
        timebase: &mut Timebase,
    ) -> Result<(), String>;

    fn has_global_tick(&self) -> bool {
        false
    }

    fn global_tick(&mut self, _gl: &glow::Context, _world: &mut World) -> Result<AppPhase, String> {
        Err("Global tick not implemented for this type".to_string())
    }
}
