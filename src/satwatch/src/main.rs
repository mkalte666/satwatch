mod components;
mod rendering;
mod space;
mod util;

use imgui::Context;

use sdl2::{
    event::Event,
    video::{GLProfile, Window},
    Sdl,
};

use imgui_glow_renderer::AutoRenderer;

use crate::rendering::renderer::Renderer;
use crate::space::world_control::WorldControl;
use crate::util::input_events::sdl_to_our_event;
use crate::util::sdl2_imgui_tmpfix::SdlPlatform;
use crate::util::vertex_tools;
use glam::f32::*;
use glow::HasContext;
use legion::*;
use sdl2::keyboard::Keycode;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    use sdl2::image::{LoadSurface, Sdl2ImageContext};
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

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut imgui_renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut world = World::default();
    let mut render_system = crate::rendering::renderer::Renderer::create();
    let mut world_control = WorldControl::new();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::WindowEvent;
            platform.handle_event(&mut imgui, &event);
            if let Event::Quit { .. } = event {
                break 'main_loop;
            } else if let Event::Window { win_event, .. } = event {
                if let WindowEvent::Resized(w, h) = win_event {
                    unsafe {
                        imgui_renderer.gl_context().viewport(0, 0, w, h);
                    }
                }
            } else {
                let e = sdl_to_our_event(event);
                world_control.handle_input(imgui_renderer.gl_context(), &mut world, e);
            }
        }

        // world tick here
        world_control.tick(imgui_renderer.gl_context(), &mut world);

        if let Err(e) = render_system.load(imgui_renderer.gl_context(), &mut world) {
            eprintln!("{}", e);
        }
        platform.prepare_frame(&mut imgui, &window, &event_pump);
        let mut ui = imgui.frame();

        use imgui::*;
        world_control.ui(imgui_renderer.gl_context(), &mut world, &mut ui);

        let draw_data = imgui.render();

        unsafe {
            imgui_renderer
                .gl_context()
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        let (w, h) = window.drawable_size();
        let aspect = (w as f32) / (h as f32);
        render_system.draw(imgui_renderer.gl_context(), &mut world, aspect);
        imgui_renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }

    Ok(())
}
