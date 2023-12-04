use sdl2::{ render::{ WindowCanvas, Texture }, rect::Rect, pixels::Color };

use super::{ playing_card::Card, game::Game };

const BACKGROUND_COLOUR: Color = Color::RGB(22, 154, 24);

fn render_card(canvas: &mut WindowCanvas, texture: &Texture, card: &Card) -> Result<(), String> {
    let screen_rect = Rect::from_center(card.position, card.sprite.width(), card.sprite.height());

    canvas.copy(texture, None, screen_rect)?;

    Ok(())
}

fn render_playing_columns(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for column in &game.playing_columns {
        for card in column {
            match card.visible {
                true => render_card(canvas, &card.front_texture, &card)?,
                false => render_card(canvas, &card.back_texture, &card)?,
            }
        }
    }

    Ok(())
}

fn render_draw_deck(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    if game.draw_deck.len() > 0 {
        let card = game.draw_deck[0];
        render_card(canvas, &card.back_texture, &card)?;
    }
    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    canvas.set_draw_color(BACKGROUND_COLOUR);
    canvas.clear();
    render_playing_columns(canvas, game)?;
    render_draw_deck(canvas, game)?;
    canvas.present();
    Ok(())
}
