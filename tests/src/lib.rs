use ::Cards::*;
use ::Game::*;

#[cfg(test)]
mod tests {
    mod dealer {
        mod checks {
            use crate::*;

            #[test]
            fn stand() {
                let hand = vec![Card {
                    value: Value::Queen,
                    suit: Suit::Spades,
                    hidden: false,
                }];

                let hidden = Card {
                    value: Value::King,
                    suit: Suit::Spades,
                    hidden: true,
                };

                match <Dealer as User>::from(hand, Some(hidden), None) {
                    Ok(mut dealer) => {
                        let mut player = Player::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        assert_eq!(game.dealer_checks(), Ok((false, false)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }

            #[test]
            fn hit() {
                let hand = vec![Card {
                    value: Value::Queen,
                    suit: Suit::Spades,
                    hidden: false,
                }];

                let hidden = Card {
                    value: Value::Six,
                    suit: Suit::Clubs,
                    hidden: true,
                };

                match <Dealer as User>::from(hand.clone(), Some(hidden), None) {
                    Ok(mut dealer) => {
                        let mut player = Player::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        game.shuffle();
                        let _ = game.dealer_checks();

                        assert_ne!(dealer.hand, hand);
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }

            #[test]
            fn bust() {
                let hand = vec![
                    Card {
                        value: Value::Queen,
                        suit: Suit::Spades,
                        hidden: false,
                    },
                    Card {
                        value: Value::Queen,
                        suit: Suit::Hearts,
                        hidden: false,
                    },
                ];

                let hidden = Card {
                    value: Value::Six,
                    suit: Suit::Clubs,
                    hidden: true,
                };

                match <Dealer as User>::from(hand.clone(), Some(hidden), None) {
                    Ok(mut dealer) => {
                        let mut player = Player::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);

                        assert_eq!(game.dealer_checks(), Ok((false, true)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }

            #[test]
            fn won() {
                let hand = vec![Card {
                    value: Value::Queen,
                    suit: Suit::Spades,
                    hidden: false,
                }];

                let hidden = Card {
                    value: Value::Ace,
                    suit: Suit::Hearts,
                    hidden: true,
                };

                match <Dealer as User>::from(hand.clone(), Some(hidden), None) {
                    Ok(mut dealer) => {
                        let mut player = Player::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        assert_eq!(game.dealer_checks(), Ok((true, false)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }
        }
    }

    mod player {
        mod checks {
            use crate::*;

            #[test]
            fn busted() {
                let hand = vec![
                    Card {
                        value: Value::Queen,
                        suit: Suit::Spades,
                        hidden: false,
                    },
                    Card {
                        value: Value::Jack,
                        suit: Suit::Spades,
                        hidden: false,
                    },
                    Card {
                        value: Value::King,
                        suit: Suit::Spades,
                        hidden: false,
                    },
                ];
                match <Player as User>::from(hand, None, Some(30)) {
                    Ok(mut player) => {
                        let mut dealer = Dealer::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        assert_eq!(game.player_checks(), Ok((false, true)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }

            #[test]
            fn won() {
                let hand = vec![
                    Card {
                        value: Value::Ace,
                        suit: Suit::Spades,
                        hidden: false,
                    },
                    Card {
                        value: Value::Jack,
                        suit: Suit::Diamonds,
                        hidden: false,
                    },
                ];
                match <Player as User>::from(hand, None, Some(30)) {
                    Ok(mut player) => {
                        let mut dealer = Dealer::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        assert_eq!(game.player_checks(), Ok((true, false)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }

            #[test]
            fn under() {
                let hand = vec![
                    Card {
                        value: Value::Nine,
                        suit: Suit::Hearts,
                        hidden: false,
                    },
                    Card {
                        value: Value::Jack,
                        suit: Suit::Diamonds,
                        hidden: false,
                    },
                ];
                match <Player as User>::from(hand, None, Some(30)) {
                    Ok(mut player) => {
                        let mut dealer = Dealer::new();
                        let mut game = Game::new(1, &mut dealer, &mut player);
                        assert_eq!(game.player_checks(), Ok((false, false)));
                    }
                    Err(e) => println!("Errors Occurred: {:?}", e),
                }
            }
        }
    }

    mod deck {
        mod suits {
            use crate::*;

            #[test]
            fn removing_cards_when_dealt() {
                let mut dealer = Dealer::new();
                let mut player = Player::new();
                let mut game = Game::new(1, &mut dealer, &mut player);

                game.deal();

                assert_eq!(game.deck.cards.len(), 48);
            }

            #[test]
            fn count_hearts() {
                let deck = Deck::new(1);
                let count = deck
                    .cards
                    .iter()
                    .filter(|card| card.suit == Suit::Hearts)
                    .count();

                assert_eq!(count, 13);
            }

            #[test]
            fn count_spades() {
                let deck = Deck::new(1);
                let count = deck
                    .cards
                    .iter()
                    .filter(|card| card.suit == Suit::Spades)
                    .count();

                assert_eq!(count, 13);
            }

            #[test]
            fn count_diamonds() {
                let deck = Deck::new(1);
                let count = deck
                    .cards
                    .iter()
                    .filter(|card| card.suit == Suit::Diamonds)
                    .count();

                assert_eq!(count, 13);
            }

            #[test]
            fn count_clubs() {
                let deck = Deck::new(1);
                let count = deck
                    .cards
                    .iter()
                    .filter(|card| card.suit == Suit::Clubs)
                    .count();

                assert_eq!(count, 13);
            }
        }
    }

    mod values {
        use crate::{Deck, Suit, Value::*};

        #[test]
        fn test_card_counts() {
            let deck = Deck::new(2);
            let expected_count = 8;

            let all_values = [
                Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
            ];

            for value in &all_values {
                let count = deck
                    .cards
                    .iter()
                    .filter(|card| card.value == *value)
                    .count();
                assert_eq!(
                    count, expected_count,
                    "Expected {} of {:?}, but found {}",
                    expected_count, value, count
                );
            }
        }

        #[test]
        fn test_card_counts_suit() {
            let deck = Deck::new(2);
            let expected_count = 2;

            let all_values = [
                Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
            ];

            for suit in [Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades] {
                for value in &all_values {
                    let count = deck
                        .cards
                        .iter()
                        .filter(|card| card.value == *value && card.suit == suit)
                        .count();
                    assert_eq!(
                        count, expected_count,
                        "Expected {} of {:?}, but found {}",
                        expected_count, value, count
                    );
                }
            }
        }

        #[test]
        fn test_card_number() {
            let deck = Deck::new(3);
            let count = deck.cards.iter().count();

            assert_eq!(count, 52 * 3);
        }
    }
}
