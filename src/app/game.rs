use rand::{ seq::SliceRandom, thread_rng };
use sdl2::rect::Point;

use super::playing_card::{ Card, Deck };

pub struct Game<'a> {
    pub playing_columns: [Vec<Card<'a>>; 7],
    pub draw_deck: Vec<Card<'a>>,
    pub drawn_cards: Vec<Card<'a>>,
    used_draw_cards: Vec<Card<'a>>,
    pub winning_stacks: [Vec<Card<'a>>; 4],
}

impl<'a> Game<'a> {
    pub fn new(deck: &mut Deck<'a>) -> Self {
        let empty_stacks: [Vec<Card<'a>>; 4] = [vec![], vec![], vec![], vec![]];
        let mut playing_columns: [Vec<Card<'a>>; 7] = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let mut draw_deck: Vec<Card<'a>> = vec![];
        let drawn_cards: Vec<Card<'a>> = vec![];
        let used_draw_cards: Vec<Card<'a>> = vec![];

        //randomise deck
        deck.cards.shuffle(&mut thread_rng());

        initialise_playing_columns(&mut playing_columns, deck);

        initialise_draw_deck(&mut draw_deck, deck);

        Game {
            playing_columns,
            draw_deck,
            drawn_cards,
            used_draw_cards,
            winning_stacks: empty_stacks,
        }
    }
}

fn initialise_playing_columns<'a>(playing_columns: &mut [Vec<Card<'a>>; 7], deck: &mut Deck<'a>) {
    for (i, column) in playing_columns.iter_mut().enumerate() {
        for j in 0..i + 1 {
            let mut card = deck.cards.pop().expect("no card to pop");
            card.position = Point::new(100 * (i as i32) + 91, 20 * (j as i32) + 201);
            if j == i {
                card.visible = true;
            }
            column.push(card);
        }
    }
}

fn initialise_draw_deck<'a>(draw_deck: &mut Vec<Card<'a>>, deck: &mut Deck<'a>) {
    for _ in 0..deck.cards.len() {
        let mut card = deck.cards.pop().expect("no card to pop");
        card.position = Point::new(600, 100);
        draw_deck.push(card);
    }
}
