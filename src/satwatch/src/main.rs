mod components;
mod download;
mod main_loop;
mod rendering;
mod util;
mod world;

use sdl2::video::{GLProfile, Window};

use imgui_glow_renderer::AutoRenderer;

use crate::main_loop::app_phase::AppPhase;
use crate::util::imgui_logger::*;

use imgui_sdl2_support::SdlPlatform;

use glow::HasContext;
use legion::*;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn log_level() {
    log::set_max_level(log::LevelFilter::Debug);
    #[cfg(feature = "trace_logging")]
    log::set_max_level(log::LevelFilter::Trace);
}

fn main() -> Result<(), String> {
    log_level();
    let imgui_logger = ImguiLoggerUi::init();

    let sdl = sdl2::init()?;
    let _image = sdl2::image::init(sdl2::image::InitFlag::all())?;

    let video_subsystem = sdl.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window("SatWatch", 1024, 768)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context()?;
    window.gl_make_current(&gl_context)?;
    window.subsystem().gl_set_swap_interval(1)?;

    let gl = glow_context(&window);
    unsafe {
        gl.enable(glow::DEPTH_TEST);
        gl.enable(glow::MULTISAMPLE);
    }

    let mut imgui = imgui::Context::create();
    imgui.io_mut().config_flags.set(imgui::ConfigFlags::DOCKING_ENABLE, true);
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let platform = SdlPlatform::init(&mut imgui);
    let imgui_renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let event_pump = sdl.event_pump().unwrap();

    let world = World::default();
    let render_system = crate::rendering::renderer::Renderer::create();

    let mut lp = main_loop::MainLoopData {
        phase: AppPhase::Downloads,
        imgui_logger,
        window,
        platform,
        imgui,
        imgui_renderer,
        event_pump,
        world,
        render_system,
        uis: Vec::new(),
    };
    lp.run();

    Ok(())
}
