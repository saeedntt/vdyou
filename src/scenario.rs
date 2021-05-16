use crate::character::Character;
use crate::mshape::MShape;

pub struct ScreenPlay<'a> {
    characters: Vec<Character>,
    events: Vec<Event<'a>>,
}

struct Event<'a> {
    characters: Vec<&'a Character>,
    act: Vec<dyn Act>,
}

pub trait Act {}

pub struct Move {
    to_x: f32,
    to_y: f32,
}

impl Move {
    fn new(to_x: f32, to_y: f32) -> Self {
        Move {
            to_x,
            to_y,
        }
    }
}

impl Act for Move {}

impl ScreenPlay {
    fn new() -> Self {
        ScreenPlay {
            characters: vec![],
            events: vec![],
        }
    }

    fn new_character(&mut self, character: Character) {
        self.characters.push(character);
    }

    fn get_character_names(&self) -> Vec<&String> {
        self.characters.iter().map(|char| char.name).collect()
    }

    fn move_character(&mut self, character: &String, to_x: f32, to_y: f32) {
        let x = self.characters
            .iter()
            .find(|&&pr| pr.name == character).expect("");
        self.events.push(Event {
            characters: vec![x],
            act: vec![Move::new(to_x, to_y)],
        })
    }
}