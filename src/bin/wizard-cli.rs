use wizard::components::deck::Deck;
use wizard::components::player::*;
use wizard::components::util::Util;
use wizard::*;

fn main() {
    Util::print_wizard_ascii_art();

    let new_deck = Deck::build();

    let mut players = get_players();
    Player::print_names(&players);

    Util::press_enter_to_("start first round");

    let num_rounds = new_deck.len() / players.len();
    for round_num in 1..(num_rounds + 1) {
        let mut deck = Deck(Util::shuffle_vec(new_deck.0.clone()));

        // Get players and rotate dealer.
        let player_rotation = round_num - 1 % players.len();
        players.rotate_left(player_rotation);
        let dealer = players[0].clone();
        let leader = players[1].clone();

        // Rotate again so leader receives first deal and starts betting round.
        players.rotate_left(1);

        // Deal cards and reset stats.
        players.iter_mut().for_each(|player| {
            player.hand.clear();
            player.tricks = 0;
            for _ in 0..round_num {
                player.hand.push(deck.pop().unwrap());
            }
        });

        println!(
            "\n--- Round {:>2} --- \nDealer: {} \nLeader: {}",
            round_num, dealer.name, leader.name
        );
        for player in &players {
            if player.operator == Operator::Human {
                println!("\nYour hand: {}\n", player.hand);
                break;
            }
        }

        let trump = match deck.pop() {
            Some(card) => set_trump(card, &dealer),
            None => {
                println!("No Trump!");
                WIZARD
            }
        };
        println!("--------------------");

        Util::press_enter_to_("start betting");

        place_bets(&mut players);
        Util::press_enter_to_("play first trick");

        play_tricks(&mut players, trump);
        calc_score(&mut players);
        // Reset player order to original so scoreboard and dealer rotation are consistent.
        while players[0].original_position != 0 {
            players.rotate_left(1);
        }

        println!("End of round #{} - Current standings:", round_num);
        Player::print_score(&players);

        Util::press_enter_to_("start next round");
    }

    let winner = calc_winner(players);
    println!(
        "{} is the winner with {} points!",
        winner.name, winner.score
    );
}
