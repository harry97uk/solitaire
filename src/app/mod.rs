mod playing_card;
mod game;
mod render;

use crate::app::render::render;
use std::time::Duration;

use sdl2::{ render::Canvas, video::Window, event::Event, keyboard::Keycode, Sdl };

use crate::app::playing_card::{
    create_card_front_textures,
    create_card_back_texture,
    initialise_cards,
    Deck,
};

use self::game::Game;

pub fn run_application(sdl_context: Sdl, mut canvas: Canvas<Window>) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let card_textures = create_card_front_textures(&texture_creator);
    let back_texture = create_card_back_texture(&texture_creator);

    let mut deck: Deck<'_> = initialise_cards(&card_textures, &back_texture);

    let game = Game::new(&mut deck);

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
        render(&mut canvas, &game)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
