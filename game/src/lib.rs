use std::io::{stdin, stdout, Write};

use Cards::*;

#[derive(Debug, PartialEq)]
pub enum Errs {
    CardOutOfRange,
    SuitOutRange,
    UnknownError,
    NoHiddenProvided,
    HiddenProvided,
    NoBankProvided,
    BankProvided,
}

pub trait User: Sized {
    fn new() -> Self;
    fn deal(&mut self, deck: &mut Vec<Card>) -> Result<[Card; 2], Errs>;
    fn hit(&mut self, deck: &mut Vec<Card>) -> Result<Card, Errs>;
    fn stand(&mut self, hand: &Vec<Card>) -> Result<u32, Errs>;
    fn from(hand: Vec<Card>, hidden: Option<Card>, bank: Option<i32>) -> Result<Self, Errs>;
}

pub struct Game<'a> {
    pub deck: Deck,
    dealer: &'a mut Dealer,
    player: &'a mut Player,
    pot: i32,
    side_pot: i32,
}

impl<'a> Game<'a> {
    pub fn new(num_decks: u32, dealer: &'a mut Dealer, player: &'a mut Player) -> Self {
        Self {
            deck: Deck::new(num_decks),
            dealer,
            player,
            pot: 0,
            side_pot: 0,
        }
    }

    pub fn shuffle(&mut self) {
        self.deck = self.deck.shuffle();
    }

    pub fn deal(&mut self) {
        self.shuffle();

        match self.dealer.deal(&mut self.deck.cards) {
            Ok([c1, c2]) => {
                self.dealer.hand.push(c1);
                self.dealer.hidden = c2;
            }
            Err(e) => {
                println!("Errors occured: {:?}", e);
            }
        }

        match self.player.deal(&mut self.deck.cards) {
            Ok([c1, c2]) => {
                self.player.hand.extend([c1, c2]);
            }
            Err(e) => {
                println!("Errors occured: {:?}", e)
            }
        }
    }

    pub fn player_hit(&mut self) -> (bool, bool) {
        match self.player.hit(&mut self.deck.cards) {
            Ok(card) => {
                self.player.hand.push(card);
                match self.player_checks() {
                    Ok(results) => results,
                    Err(e) => {
                        println!("Error Occured: {:?}", e);
                        (false, false)
                    }
                }
            }

            Err(e) => {
                println!("Error Occured: {:?}", e);
                (false, false)
            }
        }
    }

    pub fn dealer_hit(&mut self) -> (bool, bool) {
        match self.dealer.hit(&mut self.deck.cards) {
            Ok(card) => {
                self.dealer.hand.push(card);
                match self.dealer_checks() {
                    Ok(results) => results,
                    Err(e) => {
                        println!("Error Occured: {:?}", e);
                        (false, false)
                    }
                }
            }

            Err(e) => {
                println!("Error Occured: {:?}", e);
                (false, false)
            }
        }
    }

    pub fn dealer_reveal(&mut self) {
        self.dealer.show();
    }

    pub fn print(&mut self) {
        println!("Bank: ${}", self.get_bank());
        println!("\n\n");
        print_cards_side_by_side(&self.dealer.hand, Some(self.dealer.hidden));
        println!("\n\n");
        print_cards_side_by_side(&self.player.hand, None);
    }

    pub fn clear(&mut self) {
        self.dealer.hand.clear();
        self.player.hand.clear();
    }

    pub fn new_deck(&mut self, num_decks: u32) {
        self.deck = Deck::new(num_decks);
        self.shuffle();
    }

    pub fn ask_insurance(&mut self) -> bool {
        if self.dealer.hand[0].value == Value::Ace {
            loop {
                print!("Do you wnt insurance? (y/n): ");
                stdout().flush().unwrap();

                let mut inp = String::new();

                stdin().read_line(&mut inp).unwrap();

                match inp.trim() {
                    "y" | "Y" => {
                        self.side_pot = (self.pot as f32 / 2.0).floor() as i32;
                        self.remove_insurance();
                        return true;
                    }
                    "n" | "N" => return false,

                    _ => {
                        println!("That is not an input!");
                    }
                }
            }
        }

        false
    }

    pub fn get_bank(&mut self) -> i32 {
        self.player.bank
    }

    pub fn add_bank(&mut self, bj: bool) {
        if bj {
            self.player.bank += (self.pot as f32 * 2.5).floor() as i32;
        } else {
            self.player.bank += self.pot * 2;
        }
    }

    pub fn reset_bank(&mut self) {
        self.pot = 0;
        self.side_pot = 0;
    }

    pub fn revert_bank(&mut self, i: bool) {
        if !i {
            self.player.bank += self.pot;
        } else {
            self.player.bank += self.pot + self.side_pot;
        }
    }

    pub fn remove_bank(&mut self, value: i32) {
        self.player.bank -= value;

        if self.player.bank < 0 {
            self.player.bank = 0;
        }

        self.pot = value;
    }

    pub fn remove_insurance(&mut self) {
        self.player.bank -= self.side_pot;
    }

    pub fn cards_left(&mut self) -> usize {
        self.deck.cards.len()
    }

    pub fn wait_for_seconds(&mut self, seconds: u64) {
        print!("\x1B[2J\x1B[1;1H");
        wait_for_seconds(seconds);
    }

    pub fn totals(&mut self) -> Result<(i32, i32), Errs> {
        let player_total = calculate_total(&self.player.hand)?;

        let mut dh = self.dealer.hand.clone();
        dh.push(self.dealer.hidden);

        let dealer_total = calculate_total(&dh)?;
        Ok((player_total, dealer_total))
    }

    pub fn player_checks(&mut self) -> Result<(bool, bool), Errs> {
        if self.player.hand.len() == 5 {
            return Ok((true, false));
        }

        match calculate_total(&self.player.hand) {
            Ok(total) => {
                if total < 21 {
                    Ok((false, false))
                } else if total > 21 {
                    Ok((false, true))
                } else {
                    Ok((true, false))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn dealer_checks(&mut self) -> Result<(bool, bool), Errs> {
        if self.has_blackjack() {
            return Ok((true, true));
        }

        loop {
            let mut hand = self.dealer.hand.clone();
            hand.push(self.dealer.hidden);

            let total = calculate_total(&hand)?;

            if total < 17 {
                wait_for_seconds(1);

                match self.dealer.hit(&mut self.deck.cards) {
                    Ok(card) => self.dealer.hand.push(card),
                    Err(e) => return Err(e),
                }
            } else {
                if total > 21 {
                    // Busted
                    return Ok((false, true));
                } else if total == 21 {
                    // Got 21 (NOT A BLACKJACK)!!!
                    return Ok((true, false));
                } else {
                    // Resume Play
                    return Ok((false, false));
                }
            }
            print!("\x1B[2J\x1B[1;1H");
            self.print();
        }
    }

    pub fn has_blackjack(&mut self) -> bool {
        let mut hand = self.dealer.hand.clone();
        hand.push(self.dealer.hidden);
        if calculate_total(&hand) == Ok(21) && hand.len() == 2 {
            true
        } else {
            false
        }
    }

    pub fn player_stand(&mut self) {
        match self.player.stand(&self.player.hand.clone()) {
            Ok(total) => {
                println!("Your total is {}", total);
            }

            Err(e) => {
                println!("An Error Occured: {:?}", e);
            }
        }
    }

    pub fn dealer_stand(&mut self) {
        self.dealer.hand.push(self.dealer.hidden);
        match self.player.stand(&self.dealer.hand.clone()) {
            Ok(total) => {
                println!("{}", total);
            }

            Err(e) => {
                println!("An Error Occured: {:?}", e);
            }
        }
    }
}

pub struct Dealer {
    pub hand: Vec<Card>,
    hidden: Card,
}

impl Dealer {
    pub fn show(&mut self) {
        let old = self.hidden.clone();
        self.hidden = Card {
            value: old.value,
            suit: old.suit,
            hidden: false,
        };
    }
}

impl User for Dealer {
    fn new() -> Self {
        Self {
            hand: Vec::new(),
            hidden: Card::new(Value::Nil, Suit::Nil, true),
        }
    }

    fn from(hand: Vec<Card>, hidden: Option<Card>, bank: Option<i32>) -> Result<Self, Errs> {
        if let Some(_) = bank {
            Err(Errs::BankProvided)
        } else if let Some(h) = hidden {
            Ok(Self { hand, hidden: h })
        } else {
            Err(Errs::NoHiddenProvided)
        }
    }

    fn deal(&mut self, deck: &mut Vec<Card>) -> Result<[Card; 2], Errs> {
        let mut hand = Vec::with_capacity(2);

        for i in 0..2 {
            let card = deck.pop().expect("Failed to pop card of the deck");

            let new_card = Card {
                value: card.value,
                suit: card.suit,
                hidden: i != 0,
            };

            hand.push(new_card);
        }

        if let [c1, c2] = hand[..] {
            Ok([c1, c2])
        } else {
            Err(Errs::UnknownError)
        }
    }

    fn hit(&mut self, deck: &mut Vec<Card>) -> Result<Card, Errs> {
        match deck.pop() {
            Some(card) => Ok(card),
            None => Err(Errs::UnknownError),
        }
    }

    fn stand(&mut self, hand: &Vec<Card>) -> Result<u32, Errs> {
        match calculate_total(hand) {
            Ok(total) => Ok(total as u32),

            Err(e) => Err(e),
        }
    }
}

pub struct Player {
    pub bank: i32,
    pub hand: Vec<Card>,
}

impl User for Player {
    fn new() -> Self {
        Self {
            bank: 1000,
            hand: Vec::new(),
        }
    }

    fn from(hand: Vec<Card>, hidden: Option<Card>, bank: Option<i32>) -> Result<Self, Errs> {
        if let Some(_) = hidden {
            Err(Errs::HiddenProvided)
        } else if let Some(b) = bank {
            Ok(Self { bank: b, hand })
        } else {
            Err(Errs::NoBankProvided)
        }
    }

    fn deal(&mut self, deck: &mut Vec<Card>) -> Result<[Card; 2], Errs> {
        let mut hand: Vec<Card> = Vec::with_capacity(2);

        for _ in 0..2 {
            let card = deck.pop().expect("Failed to pop card of the deck");

            hand.push(card);
        }

        if let [c1, c2] = hand[..] {
            Ok([c1, c2])
        } else {
            Err(Errs::UnknownError)
        }
    }

    fn hit(&mut self, deck: &mut Vec<Card>) -> Result<Card, Errs> {
        match deck.pop() {
            Some(card) => Ok(card),
            None => Err(Errs::UnknownError),
        }
    }

    fn stand(&mut self, hand: &Vec<Card>) -> Result<u32, Errs> {
        match calculate_total(hand) {
            Ok(total) => Ok(total as u32),

            Err(e) => Err(e),
        }
    }
}

fn print_cards_side_by_side(hand: &Vec<Card>, hidden: Option<Card>) {
    let mut cards = hand.clone();

    if hidden != None {
        cards.insert(1, hidden.expect("Failed to push hidden"));
    }

    let card_count = cards.len();

    let mut top_line = vec![];
    let mut line1 = vec![];
    let mut line2 = vec![];
    let mut line3 = vec![];
    let mut line4 = vec![];
    let mut bottom_line = vec![];

    for i in 0..card_count {
        if cards[i].hidden == true {
            // Last card: face-down with centered "HUGOS" and "CARDS"
            top_line.push("+-------+".to_string());
            line1.push("|       |".to_string());
            line2.push("| HUGOS |".to_string());
            line3.push("| CARDS |".to_string());
            line4.push("|       |".to_string());
            bottom_line.push("+-------+".to_string());
        } else {
            let label = cards[i].short_label();
            top_line.push("+-------+".to_string());
            line1.push(format!("| {:<3}   |", label));
            line2.push("|       |".to_string());
            line3.push("|       |".to_string());
            line4.push(format!("|   {:>3} |", label));
            bottom_line.push("+-------+".to_string());
        }
    }

    // Print all lines joined by spaces
    println!();
    println!("{}", top_line.join("  "));
    println!("{}", line1.join("  "));
    println!("{}", line2.join("  "));
    println!("{}", line3.join("  "));
    println!("{}", line4.join("  "));
    println!("{}", bottom_line.join("  "));

    cards.pop();
}

fn calculate_total(hand: &Vec<Card>) -> Result<i32, Errs> {
    let mut total = 0;
    for card in hand {
        match card.value {
            Value::Ace => {
                total += 11;
            }

            Value::Two => {
                total += 2;
            }

            Value::Three => {
                total += 3;
            }

            Value::Four => {
                total += 4;
            }

            Value::Five => {
                total += 5;
            }

            Value::Six => {
                total += 6;
            }

            Value::Seven => {
                total += 7;
            }

            Value::Eight => {
                total += 8;
            }

            Value::Nine => {
                total += 9;
            }

            Value::Ten | Value::Jack | Value::King | Value::Queen => {
                total += 10;
            }

            Value::Nil => {
                return Err(Errs::UnknownError);
            }
        }
    }

    if total > 21 {
        for card in hand.iter() {
            if card.value == Value::Ace && total > 21 {
                total -= 10;
            }
        }
    }

    Ok(total)
}

fn wait_for_seconds(seconds: u64) {
    std::thread::sleep(std::time::Duration::from_secs(seconds));
}
