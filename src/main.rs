use std::cmp::Ordering::*;
use std::io::{stdin, stdout, Write};

use crossterm::event::{self, Event};

use ::Game::{Dealer, Game, Player, User};

fn main() {
    let mut dealer = Dealer::new();
    let mut player = Player::new();

    let mut game = Game::new(2, &mut dealer, &mut player);

    game.deal();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        if game.cards_left() <= 14 {
            println!("There are no cards left in the deck");
            game.new_deck(2);
        } else if game.get_bank() == 0 {
            println!("The House wins!");
            break;
        }
        let out = place_bets(&mut game);

        if out {
            println!("Goodbye!");
            return;
        }

        game.print();
        let has_insurance = game.ask_insurance();

        loop {
            if has_insurance {
                if game.has_blackjack() {
                    game.revert_bank(true);
                    game.dealer_reveal();
                    game.print();
                    println!("The dealer had a blackjack!");
                    wait_for_input();
                    break;
                } else {
                    println!("The dealer didn't have a blackjack :(");
                    wait_for_input();
                }
            }

            print!("\x1B[2J\x1B[1;1H");
            println!("There are {} cards left in the deck.", game.cards_left());
            game.print();

            if game.player_checks() == Ok((true, false)) {
                println!("YOU GOT A BLACKJACK!");
                game.add_bank(true);
                wait_for_input();
                break;
            }

            print!("Enter an input (hit, stand): ");
            stdout().flush().unwrap();

            let mut response = String::new();
            stdin()
                .read_line(&mut response)
                .expect("Failed to read input");

            match response.trim() {
                "hit" => {
                    let (won, bust) = game.player_hit();

                    if won || bust {
                        if won {
                            print!("\x1B[2J\x1B[1;1H");
                            game.print();
                            println!("YOU GOT 21!!!!!!!");
                            game.add_bank(false);
                        }

                        if bust {
                            print!("\x1B[2J\x1B[1;1H");
                            game.print();
                            println!("BUSTED!!!!");
                        }

                        wait_for_input();
                        break;
                    }
                }

                "stand" => {
                    stand(&mut game);
                    break;
                }

                _ => {
                    println!("Invalid input! Try 'hit' or 'stand'");
                }
            }
        }

        game.reset_bank();
        restart(&mut game);
    }
}

fn wait_for_input() {
    println!("Press [ENTER]! to continue: ");
    stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(0));

    loop {
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(_) = event::read().unwrap() {
                break;
            }
        }
    }
    print!("\x1B[2J\x1B[1;1H");
}

fn stand(game: &mut Game) {
    game.player_stand();
    game.wait_for_seconds(1);
    print!("\x1B[2J\x1B[1;1h");
    game.print();
    game.dealer_reveal();
    game.wait_for_seconds(1);
    game.print();
    print!("\x1B[2J\x1B[1;1H");
    game.print();

    let result = game.dealer_checks();
    if let Ok(vals) = result {
        match vals {
            (false, true) => {
                println!("THEY BUSTED EVERYWHERE!!!!");
                game.add_bank(false);
            }
            (true, false) => {
                println!("THEY GOT 21!!!!!!!!!!!!!!!");
            }
            (false, false) => match game.totals() {
                Ok((pt, dt)) => {
                    println!("The results were: YOU: {}, DEALER: {}", pt, dt);
                    match pt.cmp(&dt) {
                        Less => println!("YOU LOST :("),
                        Greater => {
                            println!("YOU WON! :)");
                            game.add_bank(false);
                        }
                        Equal => {
                            println!("PUSH!");
                            game.revert_bank(false);
                        }
                    }
                }
                Err(e) => {
                    println!("Errors Occured: {:?}", e);
                }
            },
            (true, true) => {
                println!("THEY GOT A BLACKJACK!");
            }
        }
    } else if let Err(e) = result {
        println!("Errors Occured: {:?}", e);
    }

    //game.print();
    wait_for_input();
    restart(game);
}

fn restart(game: &mut Game) {
    game.clear();
    game.deal();
}

fn place_bets(game: &mut Game) -> bool {
    loop {
        println!("Bank: ${}", game.get_bank());
        print!("How much are you betting OR Do you want out?: ");
        stdout().flush().unwrap();

        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();

        if inp.trim() == "out" {
            return true;
        }

        let amount = inp.trim().parse::<i32>();

        if let Err(e) = amount {
            println!("Errors occured: {:?}", e);
        } else if let Ok(amount) = amount {
            let bank = game.get_bank();

            if amount == 0 {
                println!("You cannot bet $0");
            } else {
                match bank.cmp(&amount) {
                    Less => println!("Can't place the bet. Not enough money!"),
                    Greater | Equal => {
                        println!("Bets have been placed.");
                        game.remove_bank(amount);
                        break;
                    }
                }
            }
        }
    }
    false
}
