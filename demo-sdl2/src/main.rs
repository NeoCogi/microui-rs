extern crate sdl2;
use glow::*;
mod renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use crate::renderer::Renderer;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video_subsystem.window("Window", 800, 600).opengl().build().unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let ctx = window.gl_create_context().unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _) };

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rd = Renderer::new(&gl, &microui::ATLAS_TEXTURE, 800, 600);

    'running: loop {
        unsafe {
            gl.clear_color(0.6, 0.5, 0.8, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            rd.push_rect(&gl, microui::rect(0, 0, 127, 127), microui::rect(0, 0, 128, 128), microui::color(0xFF, 0xFF, 0xFF, 0xFF));
            rd.flush(&gl);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
