//  ♠, ♥, ♣, ♦
use std::env;
use std::any::Any;
// Todo: Make Suite a printable enum.
// use std::fmt;
// enum Suite { Clubs, Diamonds, Hearts, Spades, Special }
//
// impl fmt::Display for Suite {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // The `f` value implements the `Write` trait, which is what the
//         // write! macro is expecting. Note that this formatting ignores the
//         // various flags provided to format strings.
//         write!(f, "({})", String::from(self.type_id()))
//     }
// }

enum State { Betting, Playing }

struct Card {
    name: String,
    value: usize,
    suite: String
    // suite: &'a Suite,
}

struct Player {
    name: String,
    ordinal: usize,
    score: u8,
}

struct Bet {
    player: Player,
    bet: u8,
    tricks: u8
}

struct Round {
    state: State,
    turn: usize,
    // Todo: Make reference to Player object.
    dealer: String,
    // trump: Suite,
    trump: String,
    bets: Vec<Bet>
}

fn main() {
    print_wizard_ascii_art();

    // Setup.
    let mut players = get_players();
    for player in &players {
        println!("Name: {}, Score: {}", player.name, player.score);
    }

    let num_rounds = 60 / players.len();

    let deck = get_deck();
    for card in &deck {
        println!("{:>2} {:>6} {}", card.value, card.name, card.suite);
    }

    // Rounds.
    for round_num in 0..num_rounds {
        let _current_round = round_num + 1;
        let round = Round {
            state: State::Betting,
            turn: _current_round,
            dealer: String::from(&players[0].name),
            trump: String::from("Clubs"),
            bets: vec![]
        };
        // Betting
        // Dealer
        // Order of players
    }
}

fn get_players() -> Vec<Player> {
    let mut args: Vec<String> = env::args().collect();
    // Drop the first arg as it is the command name.
    args.drain(    0..1);

    let mut players: Vec<Player> = Vec::new();
    for (ordinal, name) in args.iter().enumerate() {
        players.push(Player {
            name: name.clone(),
            score: 0,
            ordinal
        })
    }

    players
}

fn print_wizard_ascii_art() {
    println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
}

fn get_deck() -> Vec<Card> {
    // Build normal 52 card deck.
    let mut deck: Vec<Card> = Vec::new();
    let names = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
    // let suites = [Suite::Clubs, Suite::Diamonds, Suite::Hearts, Suite::Spades ];
   let suites = ["Clubs", "Diamonds", "Hearts", "Spades"];
    for suite in suites {
        for (index, name) in names.iter().enumerate() {
            deck.push( Card {
                value: index + 2,
                name: String::from(name.clone()),
                suite: String::from(suite)
            });
        }
    }

    // Add 4 Wizards.
    for i in 0..4 {
        deck.push(Card {
            name: String::from("Wizard"),
            value: 15,
            suite: String::from("Special")
        });
    }

    // Add 4 Jesters.
    for i in 0..4 {
        deck.push(Card {
            name: String::from("Jester"),
            value: 0,
            suite: String::from("Special")
            // suite: &Suite::Special
        });
    }

    println!("Unshuffled.");
    for card in &deck {
        println!("{} {} {}", card.value, card.name, card.suite);
    }

    // use rand::seq::SliceRandom;
    // use rand::thread_rng;
    // let mut shuffleable = deck.as_mut_slice();

    // for card in shuffleable {
    //     println!("{} {} {}", card.value, card.name, card.suite);
    // }

    // shuffleable.shuffle(&mut rng);
    // for card in shuffleable {
    //     println!("{} {} {}", card.value, card.name, card.suite);
    // }
    deck
}
