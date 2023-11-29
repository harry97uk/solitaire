use image_resizing::image_resizing::{ resized_image_exists, resize_image };
use sdl2::{ render::TextureCreator, video::WindowContext };
pub use sdl2::{
    rect::{ Point, Rect },
    render::{ Texture, WindowCanvas, Canvas },
    image::LoadTexture,
    video::Window,
};
pub mod image_resizing;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

pub struct Card<'a> {
    pub texture: &'a Texture<'a>,
    pub position: Point,
    pub sprite: Rect,
    pub suit: Suit,
    pub rank: Rank,
}

impl<'a> Card<'a> {
    pub fn new(texture: &'a Texture<'a>, suit: Suit, rank: Rank, offset: i32) -> Self {
        let position = Point::new(50 + offset, 50);
        // src position in the spritesheet
        let sprite = Rect::new(0, 0, texture.query().width, texture.query().height);

        Card { texture, position, sprite, suit, rank }
    }
}

// Define the Deck struct
pub struct Deck<'a> {
    pub cards: Vec<Card<'a>>,
}

pub fn create_card_textures(texture_creator: &TextureCreator<WindowContext>) -> Vec<Texture> {
    // Load individual textures for each card
    let card_textures: Vec<Texture> = (0..52)
        .map(|i| {
            let card_strings = convert_number_to_card_strings(i);
            let rank: String = card_strings.0;
            let suit: String = card_strings.1;

            let path = format!("src/assets/playing_cards/original/{}_of_{}.png", rank, suit);
            if !resized_image_exists(&path) {
                resize_image(&path);
            }
            texture_creator
                .load_texture(&path.replace("original", "resized"))
                .expect(&format!("could not load texture: {}", path))
        })
        .collect();

    card_textures
}

fn convert_number_to_card_strings(x: i32) -> (String, String) {
    let mut rank = String::from("no rank available");
    let mut suit = String::from("no suit available");
    match x {
        0..=51 => {
            rank = match x % 13 {
                0 => "ace".to_string(),
                1..=9 => ((x % 13) + 1).to_string(),
                10 => "jack".to_string(),
                11 => "queen".to_string(),
                12 => "king".to_string(),
                _ => String::from("no rank available"),
            };

            suit = match x / 13 {
                0 => "clubs".to_string(),
                1 => "diamonds".to_string(),
                2 => "hearts".to_string(),
                3 => "spades".to_string(),
                _ => String::from("no suit available"),
            };
        }
        _ => {}
    }

    (rank, suit)
}

fn convert_number_to_card_struct_enums(x: i32) -> (Rank, Suit) {
    let mut rank = Rank::Ace;
    let mut suit = Suit::Spades;

    match x {
        0..=51 => {
            rank = match x % 13 {
                0 => Rank::Ace,
                1..=9 => Rank::Number((x as u8) + 1),
                10 => Rank::Jack,
                11 => Rank::Queen,
                12 => Rank::King,
                _ => Rank::Ace,
            };

            suit = match x / 13 {
                0 => Suit::Clubs,
                1 => Suit::Diamonds,
                2 => Suit::Hearts,
                3 => Suit::Spades,
                _ => Suit::Spades,
            };
        }
        _ => {}
    }

    (rank, suit)
}

pub fn initialise_cards<'a>(card_textures: &'a Vec<Texture<'a>>) -> Deck {
    let mut cards: Vec<Card> = vec![];

    for i in 0..52 {
        let card_enums = convert_number_to_card_struct_enums(i);
        let card = Card::new(&card_textures[i as usize], card_enums.1, card_enums.0, i * 10);
        cards.push(card);
    }

    let deck = Deck { cards };

    deck
}

pub fn render_card(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    card: &Card
) -> Result<(), String> {
    let screen_rect = Rect::from_center(card.position, card.sprite.width(), card.sprite.height());

    canvas.copy(texture, None, screen_rect)?;

    Ok(())
}
