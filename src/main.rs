#![allow(temporary_cstring_as_ptr)]

use std::time::{SystemTime, UNIX_EPOCH};

mod atlas;
mod editor;
mod shader;
mod util;
mod window;

const WIDTH: u32 = 1380;
const HEIGHT: u32 = 720;
const FONT: &str = "./fonts/JetBrainsMono-Regular.ttf";
const FONT_H: u32 = 24;

fn main() {
    // ----
    // Setup SDL2 and OpenGL
    // ----
    let sdl_ctx = sdl2::init().unwrap();

    let sdl_video_subsystem = sdl_ctx.video().unwrap();
    sdl_video_subsystem.text_input().start();
    let sdl_window = sdl_video_subsystem
        .window("editor", WIDTH, HEIGHT)
        .opengl()
        .build()
        .unwrap();

    let mut sdl_events: sdl2::EventPump = sdl_ctx.event_pump().unwrap();

    // FPS 
    let sdl_timer = sdl_ctx.timer().unwrap();
    let mut start: u64;
    let mut end: u64;
    let mut elapsed: u64;
    let mut frames: u128 = 0;
    let mut start_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_millis();

    let gl_attr = sdl_video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Retaining the OpenGL context is crucial; dropping it prematurely can cause rendering issues.
    let _gl_ctx = sdl_window.gl_create_context().unwrap();

    sdl_video_subsystem.gl_set_swap_interval(0).unwrap(); // disable vsync
    gl::load_with(|s| {
        sdl_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    // ####

    // ----
    // Setup Echo
    // ----
    let mut window = window::Window::new(
        WIDTH as f32, HEIGHT as f32,
        FONT, FONT_H,
    ).unwrap();
    
    // window.theme.set_hex_cs("#fa0a1e", 123).unwrap();
    window.theme.set_hex_cs("#ffffff", 255).unwrap();
    window.theme.set_hex_bg("#030e8c").unwrap();
    window.theme.set_hex_fg("#fa0a1e").unwrap();

    'running: loop {
        start = sdl_timer.performance_counter();

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::ClearColor(
                window.theme.bg()[0],
                window.theme.bg()[1],
                window.theme.bg()[2],
                window.theme.bg()[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for sdl_event in sdl_events.poll_iter() {
            // SDL events are handled in a custom event loop that `app` understands.
            match window.handle_event(sdl_event) {
                window::WindowEvent::Quit => {
                    break 'running;
                },
                _ => { },
            }
        }

        end = sdl_timer.performance_counter();
        elapsed = ((end - start) / sdl_timer.performance_frequency()) * 1000;

        frames += 1;
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Err(_) => {}
            Ok(time) => {
                if time.as_millis() - start_now > 100 {
                    let fps = frames as f64 / ((time.as_millis() - start_now) as f64 / 1000.0);
                    window.set_fps(fps);
                    frames = 0;
                    start_now = time.as_millis();
                }
            }
        }

        window.next_frame();
        sdl_window.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_millis(1 - elapsed.min(1)));
    }
}
