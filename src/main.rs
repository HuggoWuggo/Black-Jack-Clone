use std::cmp::Ordering::*;
use std::io::{stdin, stdout, Write};

use crossterm::event::{self, Event};

use ::Game::{Dealer, Game, Player, User};

fn main() {
    let mut dealer = Dealer::new();
    let mut player = Player::new();

    let mut game = Game::new(4, &mut dealer, &mut player);

    game.deal();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        game.print();
        print!("Enter an input (hit, stand, out): ");
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
                        println!("WON!!!!!!");
                    }

                    if bust {
                        println!("BUSTED!!!!");
                    }

                    game.print();
                    wait_for_input();
                    restart(&mut game);
                }
            }

            "stand" => {
                stand(&mut game);
            }

            "out" => {
                println!("Goodbye");
                break;
            }

            _ => {
                println!("Not an input");
            }
        }
    }
}

fn wait_for_input() {
    println!("Press any key! to continue: ");
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
    wait_for_input();
    game.dealer_reveal();
    game.print();
    wait_for_input();

    let result = game.dealer_checks();
    if let Ok(vals) = result {
        match vals {
            (false, true) => {
                println!("THEY BUSTED EVERYWHERE!!!!");
            }
            (true, false) => {
                println!("THEY GOT 21!!!!!!!!!!!!!!!");
            }
            (false, false) => match game.totals() {
                Ok((pt, dt)) => {
                    println!("The results were: YOU: {}, DEALER: {}", pt, dt);
                    match pt.cmp(&dt) {
                        Less => println!("YOU LOST :("),
                        Greater => println!("YOU WON! :)"),
                        Equal => println!("PASS!"),
                    }
                }
                Err(e) => {
                    println!("Errors Occured: {:?}", e);
                }
            },
            (true, true) => {
                println!("HOW THE FUCK DID YOU GET HERE??!!!");
            }
        }
    } else if let Err(e) = result {
        println!("Errors Occured: {:?}", e);
    }

    game.print();
    wait_for_input();
    restart(game);
}

fn restart(game: &mut Game) {
    game.clear();
    game.deal();
}
