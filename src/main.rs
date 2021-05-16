#![feature(doc_alias)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// Add the "test".rs module (mod)
mod test;

fn main() -> Result<(), String> {
    println!("res: {}", test::do_something());

    let context = sdl2::init();

    // Instead of .unwrap()'ing init() above we store it in context and now we are going to deal with the two Result
    // outcomes manually, i.e. Ok(sdl2) and Err(String). Its sdl2 and String because init returns Result<sdl2, String>.
    let sdl_context = match context {
        Ok(result) => result,
        Err(message) => {
            println!(r#"SDL init error: '{}'"#, message);
            return Ok(()); // return from main()
        }
    };

    // Replacing .unwrap() with '?' is roughly equivalent to the above - except the error message is not
    // custom, its just the raw error message
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(()) // equivalent to: return Ok(());
}
