pub mod app_phase;

use crate::main_loop::app_phase::AppPhase;
use crate::rendering::renderer::Renderer;
use crate::util::imgui_logger::ImguiLoggerUi;
use crate::util::input_events::sdl_to_our_event;
use crate::world::world_ui::WorldUi;
use glow::HasContext;
use imgui::*;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use legion::*;
use libspace::timebase::Timebase;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct MainLoopData {
    pub phase: AppPhase,
    pub imgui_logger: ImguiLoggerUi,
    pub window: Window,
    pub platform: SdlPlatform,
    pub imgui: Context,
    pub imgui_renderer: AutoRenderer,
    pub event_pump: EventPump,
    pub world: World,
    pub render_system: Renderer,
    pub uis: Vec<Box<dyn WorldUi>>,
}

impl MainLoopData {
    fn repopulate_phase(&mut self) -> Result<(), String> {
        self.uis.clear();
        match self.phase {
            AppPhase::Downloads => {
                self.uis
                    .push(Box::new(crate::download::download_ui::DownloaderUi::new()));
            }
            AppPhase::Loading => {}
            AppPhase::Running => {
                Timebase::load_lsk();
                self.uis
                    .push(Box::new(crate::world::world_control::WorldControl::new(
                        self.imgui_renderer.gl_context(),
                        &mut self.world,
                    )?));
            }
        }

        Ok(())
    }
    pub fn handle_events(&mut self) -> Result<bool, String> {
        for event in self.event_pump.poll_iter() {
            use sdl2::event::WindowEvent;
            self.platform.handle_event(&mut self.imgui, &event);
            if let Event::Quit { .. } = event {
                return Ok(false);
            } else if let Event::Window { win_event, .. } = event {
                if let WindowEvent::Resized(w, h) = win_event {
                    unsafe {
                        self.imgui_renderer.gl_context().viewport(0, 0, w, h);
                    }
                }
            } else {
                let e = sdl_to_our_event(event);
                for ui in &mut self.uis {
                    ui.handle_input(self.imgui_renderer.gl_context(), &mut self.world, e);
                }
            }
        }

        Ok(true)
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.repopulate_phase();

        'main_loop: loop {
            if !self.handle_events()? {
                break 'main_loop;
            }

            let mut new_phase = self.phase;
            // updates
            for ui in &mut self.uis {
                if ui.has_global_tick() {
                    match ui.global_tick(self.imgui_renderer.gl_context(), &mut self.world) {
                        Ok(phase) => {
                            new_phase = phase;
                        }
                        Err(e) => {
                            log::warn!("Error during tick: {}", e);
                        }
                    }
                }
            }

            if let Err(e) = self
                .render_system
                .load(self.imgui_renderer.gl_context(), &mut self.world)
            {
                log::error!("Issues during render: {}", e);
            }

            self.platform
                .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);
            let mut ui = self.imgui.frame();

            ui.main_menu_bar(|| {
                self.imgui_logger.main_menu(&ui);
                for ui_t in &mut self.uis {
                    ui_t.main_menu(&ui);
                }
            });
            self.imgui_logger.draw(&mut ui);

            // drawing
            for ui_t in &mut self.uis {
                if ui_t.has_global_tick() {
                    if let Err(e) =
                        ui_t.ui(self.imgui_renderer.gl_context(), &mut self.world, &mut ui)
                    {
                        log::warn!("Error during ui draw: {}", e);
                    }
                }
            }

            // render
            let draw_data = self.imgui.render();
            unsafe {
                self.imgui_renderer
                    .gl_context()
                    .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            }
            let (w, h) = self.window.drawable_size();
            let aspect = (w as f32) / (h as f32);
            self.render_system
                .draw(self.imgui_renderer.gl_context(), &mut self.world, aspect);
            self.imgui_renderer.render(draw_data).unwrap();

            self.window.gl_swap_window();

            // phase housekeep
            if new_phase != self.phase {
                self.phase = new_phase;
                self.repopulate_phase();
            }
        }

        Ok(())
    }
}
