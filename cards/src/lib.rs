use rand::prelude::IndexedRandom;
use rand::rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub hidden: bool,
}

impl Card {
    pub fn new(value: Value, suit: Suit, hidden: bool) -> Self {
        Self {
            value,
            suit,
            hidden,
        }
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn set_suit(&mut self, suit: Suit) {
        self.suit = suit;
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn short_label(&self) -> String {
        format!("{}{}", self.value.to_string(), self.suit.to_char())
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
    Nil,
}

impl Suit {
    pub fn to_char(&self) -> char {
        match self {
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
            Suit::Spades => 's',
            Suit::Nil => '_',
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Nil,
}

impl Value {
    pub fn to_string(&self) -> &'static str {
        match self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Nil => "_",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: u32) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..num_decks {
            for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
                for value in [
                    Value::Ace,
                    Value::Two,
                    Value::Three,
                    Value::Four,
                    Value::Five,
                    Value::Six,
                    Value::Seven,
                    Value::Eight,
                    Value::Nine,
                    Value::Ten,
                    Value::Jack,
                    Value::Queen,
                    Value::King,
                ] {
                    cards.push(Card {
                        suit: suit.clone(),
                        value: value.clone(),
                        hidden: false,
                    });
                }
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) -> Deck {
        let mut nums: Vec<i32> = (0..self.cards.len() as i32).collect();
        let mut new_deck: Vec<Card> = vec![
            Card {
                value: Value::Nil,
                suit: Suit::Nil,
                hidden: false
            };
            self.cards.len()
        ];

        for card in &self.cards {
            let mut rng = rng();
            if let Some(random_element) = nums.choose(&mut rng) {
                new_deck[random_element.clone() as usize] = *card;
                nums.remove(nums.iter().position(|x| x == random_element).unwrap());
            } else {
                println!("Empty");
            }
        }

        Deck { cards: new_deck }
    }
}
