//  ♠, ♥, ♣, ♦

struct Card {
    value: usize,
    name: String,
    suite: String,
}

struct Player {
    name: String,
    score: u8,
    ordinal: u8,
}

struct Round {
    dealer: Player,
    bets: (Player, u8),
}

fn main() {
    print_wizard_ascii_art();

    // set_players();
    let num_rounds = 60 / num_players;

    let mut deck = get_deck();
    for card in deck {
        println!("{} {} {}", card.value, card.name, card.suite);
    }

    for round in 0..num_rounds {
        let current_round = round + 1;
        // Betting
        // Dealer
        // Order of players
        //


    }
}

fn set_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let names = env::args.collect();

    for (ordinal, name) in names.iter().enumerate() {
      players.push(Player {
          name,
          ordinal,
          score: 0
      })
    }

    players
}

fn print_wizard_ascii_art() {
    println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
}

fn get_num_players() -> u8 {
    let mut num_players: u8 = 0;
    while num_players < 3 || num_players > 6 {
        let mut input = String::new();

        println!("Enter the number of players (3-6):");
        std::io::stdin().read_line(&mut input).unwrap();

        let try_num_players = input.trim().parse::<u8>();
        num_players = match try_num_players {
            Ok(num) => num,
            Err(e) => {println!("not a number ({})", e); 0},
        };
    }

    num_players
}

fn get_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    let names = ["Jester", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace", "Wizard"];

    for suite in ["Clubs", "Diamonds", "Hearts", "Spades"] {
        for (index, name) in names.iter().enumerate() {
            deck.push( Card {
                value: index,
                name: String::from(name.clone()),
                suite: String::from(suite)
            });
        }
    }

    // Todo: Shuffle deck
    deck
}
