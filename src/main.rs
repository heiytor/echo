mod app;
mod atlas;
mod buffer;
mod cursor;
mod editor;
mod shader;
mod theme;
mod util;

const GO_PROGRAM: &str = r#"package main

import (
    "fmt"
    "strconv"
)

func main() {
    var input string
    var sum float64 = 0
    var count int = 0

    fmt.Println("Digite numeros e eu calcularei a media. Digite 'sair' para terminar.")

    for {
        fmt.Print("Digite um numero ou 'sair': ")
        fmt.Scan(&input)

        if input == "sair" {
            break
        }

        num, err := strconv.ParseFloat(input, 64)
        if err != nil {
            fmt.Println("Por favor, digite um numero valido ou 'sair'")
            continue
        }

        sum += num
        count++
    }

    if count > 0 {
        average := sum / float64(count)
        fmt.Printf("A media dos numeros inseridos e: %.2f\n", average)
    } else {
        fmt.Println("Nenhum numero foi inserido.")
    }
}
"#;

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const FONT: &str = "./fonts/JetBrainsMono-Regular.ttf";
    const FONT_H: u32 = 39;

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

    let gl_attr = sdl_video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Retaining the OpenGL context is crucial; dropping it prematurely can cause rendering issues.
    let _gl_ctx = sdl_window.gl_create_context().unwrap();

    gl::load_with(|s| {
        sdl_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    // ####

    // ----
    // Setup Echo
    // ----
    let mut app = app::App::new(WIDTH, HEIGHT, FONT, FONT_H).unwrap();
    app.w_theme.set_hex_bg("#030e8c").unwrap();
    app.w_theme.set_hex_fg("#ffffff").unwrap();
    // app.current_text = GO_PROGRAM.to_string();

    'running: loop {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::ClearColor(
                app.w_theme.bg()[0],
                app.w_theme.bg()[1],
                app.w_theme.bg()[2],
                app.w_theme.bg()[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut render_buffer = false;
        for sdl_event in sdl_events.poll_iter() {
            // SDL events are handled in a custom event loop that `app` understands.
            match app.handle_event(sdl_event) {
                app::WindowEvent::Quit => {
                    break 'running;
                },
                app::WindowEvent::RenderBuffer => {
                    render_buffer = true;
                }
                _ => { },
            }
        }

        // if render_buffer {
            app.render_frame(1.0);
            sdl_window.gl_swap_window();
        // }
    }
}
