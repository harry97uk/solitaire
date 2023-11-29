use sdl2::image::{ InitFlag, self };
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use solitaire::*;
use std::time::Duration;

fn render(canvas: &mut WindowCanvas, color: Color, deck: &Deck) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    for card in &deck.cards {
        render_card(canvas, &card.texture, &card)?;
    }
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("solitaire", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let card_textures = create_card_textures(&texture_creator);

    let deck = initialise_cards(&card_textures);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update

        // Render
        render(&mut canvas, Color::RGB(22, 154, 24), &deck)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
