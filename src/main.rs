use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::io;
use std::{thread, time};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    None,
}
impl Suit {
    fn symbol(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
            Suit::None => '~',
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Suit::symbol(*self))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rank {
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
    Ace,
    Wizard,
    Jester,
    None,
}
impl Rank {
    fn symbol(self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            Rank::Wizard => "W",
            Rank::Jester => "Je",
            Rank::None => "",
        }
    }
    fn value(self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
            Rank::Wizard => 15,
            Rank::Jester => 0,
            Rank::None => 0,
        }
    }
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}", Rank::symbol(*self))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}
impl Card {
    fn new() -> Card {
        Card {
            rank: Rank::None,
            suit: Suit::None,
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{}", self.rank.symbol(), self.suit.symbol())
    }
}

#[derive(Clone)]
struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn new() -> Vec<Card> {
        let mut deck: Vec<Card> = Vec::new();

        // Build normal 52 card deck.
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            for rank in ranks {
                deck.push(Card { rank, suit });
            }
        }

        // Add 4 Wizards and 4 Jesters.
        for _ in 0..4 {
            for rank in [Rank::Wizard, Rank::Jester] {
                deck.push(Card {
                    rank,
                    suit: Suit::None,
                });
            }
        }

        deck
    }

    // Todo: Make shuffle generic so both Card and Player can use it.
    pub fn shuffle(mut deck: Vec<Card>) -> Vec<Card> {
        let deck_slice = deck.as_mut_slice();
        let mut rng = thread_rng();
        deck_slice.shuffle(&mut rng);

        deck_slice.to_vec()
    }
}
impl fmt::Display for Deck {
    // Return space " " separated list of cards.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_names = String::new();
        for card in self.cards.as_slice() {
            card_names.push_str(card.rank.symbol());
            card_names.push(card.suit.symbol());
            card_names.push_str(" ")
        }

        write!(f, "{}", card_names)
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Operator {
    Human,
    Computer,
}

#[derive(Clone)]
struct Player {
    name: String,
    // The possible score range is -210..250. i16 is perfect for this.
    score: i16,
    // Bet and tricks have a range of 0..20, u5 would do.
    // Todo: Bring in external crate uX https://docs.rs/crate/ux/0.1.3t.
    bet: u8,
    tricks: u8,
    hand: Deck,
    operator: Operator,
    original_position: u8,
}
impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            score: 0,
            bet: 0,
            tricks: 0,
            hand: Deck { cards: Vec::new() },
            operator: Operator::Computer,
            original_position: 0,
        }
    }

    pub fn shuffle(mut players: Vec<Player>) -> Vec<Player> {
        let players_slice = players.as_mut_slice();
        let mut rng = thread_rng();
        players_slice.shuffle(&mut rng);

        players_slice.to_vec()
    }

    fn print_names(players: &Vec<Player>) {
        println!();
        println!(" Players");
        println!(" -------");

        players.iter().for_each(|player| {
            println!(" {}", player.name);
        });
    }

    fn print_score(players: &Vec<Player>) {
        println!();
        println!(" Name    Score   Bet   Tricks");
        println!(" ----------------------------");
        players.iter().for_each(|player| {
            println!(
                " {:8} {:>2}     {:>2}     {:>2}",
                player.name, player.score, player.bet, player.tricks
            );
        });
    }
}
impl From<&mut Player> for Player {
    fn from(player: &mut Player) -> Self {
        // Is there a better way to do this? I don't need to clone the Player, I just want to make it non-mutable.
        player.clone()
    }
}

struct Util;
impl Util {
    fn print_wizard_ascii_art() {
        println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
    }

    fn cli_next_string() -> String {
        let mut buffer = String::new();
        loop {
            io::stdin().read_line(&mut buffer).unwrap();
            match buffer.trim().parse::<String>() {
                Ok(input) => {
                    if !input.is_empty() {
                        return input;
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn cli_next_num() -> u8 {
        loop {
            match Util::cli_next_string().parse::<u8>() {
                Ok(num) => {
                    return num;
                }
                Err(_) => {
                    println!(" * Input must be a whole number * ");
                }
            }
        }
    }

    fn cli_next_pos_num() -> u8 {
        loop {
            let num = Util::cli_next_num();
            if num == 0 {
                println!(" * Input must be a positive number * ");
                continue;
            }
            return num;
        }
    }

    fn press_enter_to_(verb: &str) {
        println!("\nPress Enter to {}...", verb);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
    }

    fn sleep() {
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn main() {
    Util::print_wizard_ascii_art();

    let new_deck = Deck { cards: Deck::new() };

    let mut players = get_players();
    Player::print_names(&players);

    Util::press_enter_to_("start first round");

    // ** Short game for demo purposes. **
    // let num_rounds = new_deck.cards.len() / players.len();
    let num_rounds = 3;

    for round_num in 1..(num_rounds + 1) {
        let mut deck = Deck::shuffle(new_deck.cards.clone());

        // Get players and rotate dealer.
        let player_rotation = round_num - 1 % players.len();
        players.rotate_left(player_rotation);
        let dealer = players[0].clone();
        let leader = players[1].clone();

        // Rotate again so leader receives first deal and starts betting round.
        players.rotate_left(1);

        // Deal cards and reset stats.
        for i in 0..players.len() {
            players[i].hand.cards.clear();
            players[i].tricks = 0;
            for _ in 0..round_num {
                players[i].hand.cards.push(deck.pop().unwrap());
            }
        }

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
        let trump = set_trump(deck.pop(), &dealer);
        println!("--------------------");

        Util::press_enter_to_("start betting");

        players = place_bets(players);
        Util::press_enter_to_("play first trick");

        players = play_tricks(players, trump);
        players = calc_score(players);

        // Reset player order to original so scoreboard and dealer rotation are consistent.
        while players[0].original_position != 0 {
            players.rotate_left(1);
        }

        println!("End of round #{} - Current standings:", round_num);
        Player::print_score(&players);

        Util::press_enter_to_("start next round");
    }

    let winner = calc_final_score(players);
    println!(
        "{} is the winner with {} points!",
        winner.name, winner.score
    );
}

fn get_players() -> Vec<Player> {
    println!("Enter your name:");
    let name = Util::cli_next_string();

    let mut players = vec![Player {
        name,
        operator: Operator::Human,
        ..Player::new()
    }];

    let computer_names = ["Merlin", "Oz", "Sarumon", "Gandalf", "Kvothe"];
    for (index, name) in computer_names.iter().enumerate() {
        players.push(Player {
            name: String::from(*name),
            original_position: (index + 1) as u8,
            ..Player::new()
        });
    }

    Player::shuffle(players)
}

fn set_trump(top_card: Option<Card>, dealer: &Player) -> Card {
    let mut card = match top_card {
        Some(card) => card,
        None => Card::new(),
    };
    println!(" Trump: {}", card);

    if card.rank == Rank::Wizard {
        println!("\nTrump is a Wizard!");
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

        match dealer.operator {
            Operator::Human => {
                println!("Which suit do you select as trump?");
                for i in 0..suits.len() {
                    println!("  {}. {}", i + 1, suits[i]);
                }

                loop {
                    let selection = Util::cli_next_pos_num() as usize - 1;
                    if selection > suits.len() {
                        println!("Hey! Gotta pick what's offered here!");
                        continue;
                    }

                    card.suit = suits[selection];
                    break;
                }
            }
            Operator::Computer => {
                println!("{} will select suit...", dealer.name);
                Util::sleep();
                let rand_index = rand::thread_rng().gen_range(0..3);
                card.suit = suits[rand_index];
            }
        };

        println!("\n Trump suit: {}", card.suit);
    }
    card
}

fn place_bets(mut players: Vec<Player>) -> Vec<Player> {
    players
        .iter_mut()
        .map(|player: &mut Player| -> Player {
            let max_bet = player.hand.cards.len() as u8;

            match player.operator {
                Operator::Human => {
                    println!("What is your bet?");

                    loop {
                        player.bet = Util::cli_next_num();
                        if player.bet > max_bet {
                            println!(
                                "Yer a cocky one eh? Bet must be in the range of 0 to {}",
                                max_bet
                            );
                            continue;
                        }
                        break;
                    }

                    println!("{:>8} bet {}", player.name, player.bet);
                }
                Operator::Computer => {
                    player.bet = rand::thread_rng().gen_range(0..max_bet + 1) as u8;
                    println!("{:>8} bet {}", player.name, player.bet);
                    Util::sleep();
                }
            };
            player.into()
        })
        .collect::<Vec<Player>>()
}

fn play_tricks(mut players: Vec<Player>, trump: Card) -> Vec<Player> {
    for trick_num in 1..(players[0].hand.cards.len() + 1) {
        println!("======= Trick #{} =======", trick_num);
        let mut trick: Vec<Card> = Vec::new();
        let mut lead_suit = Suit::None;

        for i in 0..players.len() {
            // Get lead suit from first non-jester.
            for t in 0..trick.len() {
                if trick[t].suit != Suit::None {
                    lead_suit = trick[t].suit.clone();
                    break;
                }
            }

            let selected: usize = match players[i].operator {
                Operator::Human => play_trick_for_human(&players[i], lead_suit),
                Operator::Computer => play_trick_for_computer(&players[i], lead_suit),
            };

            let played_card = players[i]
                .hand
                .cards
                .drain(selected..(selected + 1))
                .last()
                .unwrap();
            println!("{:>8}: {}", players[i].name, played_card);
            trick.push(played_card);
        }

        let winner = calc_winner_of_trick(trump.suit, &trick);
        players[winner].tricks += 1;
        println!(
            "\n  Winner: {} - {}\n========================",
            trick[winner], players[winner].name
        );

        // Winner of trick should lead next trick.
        players.rotate_left(winner);

        Util::press_enter_to_("play next trick");
    }

    players
}

fn play_trick_for_human(player: &Player, lead_suit: Suit) -> usize {
    let size_of_hand = player.hand.cards.len();
    println!("\nYour hand:");

    let mut can_follow_suit = false;
    for i in 0..size_of_hand {
        println!("  {}. {}", i + 1, player.hand.cards[i]);
        if !can_follow_suit {
            let played_suit = player.hand.cards[i].suit;
            if played_suit == lead_suit {
                // Not Suit::None is needed because lead_suit is initialized
                // as Suit::None and it's possible a Wizard or Jester could match here.
                can_follow_suit = played_suit != Suit::None;
            }
        }
    }
    println!("Which card will you play?");

    loop {
        let selection = Util::cli_next_pos_num() as usize - 1;

        if selection >= size_of_hand {
            println!("Hey! Gotta pick what's offered!");
            continue;
        }

        let card = &player.hand.cards[selection];
        if card.rank == Rank::Wizard || card.rank == Rank::Jester {
            return selection;
        }

        if can_follow_suit {
            if card.suit != lead_suit {
                println!("Hey! Gotta follow suit!");
                continue;
            }
        }

        return selection;
    }
}

fn play_trick_for_computer(player: &Player, lead_suit: Suit) -> usize {
    Util::sleep();
    for x in 0..player.hand.cards.len() {
        if player.hand.cards[x].suit == lead_suit {
            return x;
        }
    }

    // Play first card in hand if suit cannot be followed.
    0
}

fn calc_winner_of_trick(trump_suit: Suit, trick: &Vec<Card>) -> usize {
    let mut winner: usize = 0;
    let mut lead_suit: Suit = trick[winner].suit;

    for i in 0..trick.len() {
        if trick[i].rank == Rank::Wizard {
            return i;
        }
        if trick[i].rank == Rank::Jester {
            continue;
        }

        // If Jester was led take suit from first non-Jester.
        if trick[winner].rank == Rank::Jester {
            if trick[i].rank != Rank::Jester {
                winner = i;
                lead_suit = trick[i].suit;
                continue;
            }
        }

        if trick[winner].suit == trump_suit {
            if trick[i].suit == trump_suit {
                if trick[i].rank.value() > trick[winner].rank.value() {
                    winner = i;
                    continue;
                }
            }
            continue;
        }

        if trick[i].suit == trump_suit {
            winner = i;
            continue;
        }

        // Follow suit...
        if trick[i].suit == lead_suit {
            if trick[i].rank.value() > trick[winner].rank.value() {
                winner = i;
            }
        }
    }

    winner
}

fn calc_score(mut players: Vec<Player>) -> Vec<Player> {
    players
        .iter_mut()
        .map(|player: &mut Player| {
            if player.tricks == player.bet {
                player.score += (2 + player.bet) as i16;
                return player.into();
            }

            let penalty: i16 = player.bet as i16 - player.tricks as i16;
            player.score -= penalty.abs();

            player.into()
        })
        .collect::<Vec<Player>>()
}

fn calc_final_score(mut players: Vec<Player>) -> Player {
    let mut winner = players.pop().unwrap();
    for player in players {
        if player.score > winner.score {
            winner = player;
        }
    }
    winner
}

#[cfg(test)]
mod tests {
    use crate::{calc_final_score, calc_score, set_trump, Player, Suit};
    use crate::{calc_winner_of_trick, Card};
    use crate::{Deck, Rank};

    #[test]
    fn test_build_deck() {
        let deck = Deck::new();
        assert_eq!(60, deck.len());
    }

    #[test]
    fn test_calc_trick() {
        let mut trump = Suit::Spade;
        let mut trick: Vec<Card> = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
        ];

        // Test all non-trump, no Wizard or Jester.
        assert_eq!(1, calc_winner_of_trick(trump, &trick));

        // Ace of lead suit should now win.
        trick.push(Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        });
        assert_eq!(3, calc_winner_of_trick(trump, &trick));

        // Low Trump should now win.
        trick.push(Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        });
        assert_eq!(4, calc_winner_of_trick(trump, &trick));

        // Higher Trump should now win.
        trick.push(Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        });
        assert_eq!(5, calc_winner_of_trick(trump, &trick));

        // First Wizard always wins.
        trick.clear();
        for _ in 0..3 {
            trick.push(Card {
                rank: Rank::Wizard,
                suit: Suit::None,
            })
        }
        assert_eq!(0, calc_winner_of_trick(trump, &trick));

        // First Jester wins if all Jesters.
        trick.clear();
        for _ in 0..3 {
            trick.push(Card {
                rank: Rank::Jester,
                suit: Suit::None,
            })
        }
        assert_eq!(0, calc_winner_of_trick(trump, &trick));

        // First non-Jester sets lead suit.
        trick.push(Card {
            rank: Rank::Two,
            suit: Suit::Diamond,
        });
        assert_eq!(3, calc_winner_of_trick(trump, &trick));

        // New lead suit is now followed.
        trick.push(Card {
            rank: Rank::Three,
            suit: Suit::Diamond,
        });
        assert_eq!(4, calc_winner_of_trick(trump, &trick));

        // Trump still wins.
        trick.push(Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        });
        assert_eq!(5, calc_winner_of_trick(trump, &trick));

        // If there is no trump then highest lead suit wins.
        trump = Suit::None;
        assert_eq!(4, calc_winner_of_trick(trump, &trick));

        // Make sure second Jester doesn't mess up lead suit.
        trick = vec![
            Card {
                rank: Rank::Jester,
                suit: Suit::None,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jester,
                suit: Suit::None,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
        ];

        trump = Suit::Diamond;
        assert_eq!(5, calc_winner_of_trick(trump, &trick));
    }

    #[test]
    fn test_calc_score() {
        let mut players: Vec<Player> = Vec::new();
        let mut player = Player::new();

        // First player bets 0 and gets 0 tricks, should have score of 3.
        players.push(player.clone());
        player.bet = 1;
        // Second player bets 1 and gets 0 tricks, should have score of -1.
        players.push(player.clone());

        player.tricks = 2;
        // Third player bets 1 and gets 2 tricks, should have score of -1.
        players.push(player.clone());

        players = calc_score(players);
        assert_eq!(players[0].score, 2);
        assert_eq!(players[1].score, -1);
        assert_eq!(players[2].score, -1);
    }

    #[test]
    fn test_set_trump() {
        let dealer = Player::new();

        // On last round deck will be empty, trump should be null card.
        let mut deck: Vec<Card> = Vec::new();
        let mut trump = set_trump(deck.pop(), &dealer);
        assert_eq!(trump, Card::new());

        // Normal card should be returned as trump.
        let two_of_hearts = Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        };
        deck.push(two_of_hearts.clone());
        trump = set_trump(deck.pop(), &dealer);
        assert_eq!(trump, two_of_hearts);

        // Nothing special happens for Jester, returned like normal card.
        let jester = Card {
            rank: Rank::Jester,
            suit: Suit::None,
        };
        deck.push(jester.clone());
        trump = set_trump(deck.pop(), &dealer);
        assert_eq!(trump, jester);

        // If Wizard is flipped the dealer should choose a suit for trump.
        let wizard = Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        };
        deck.push(wizard);
        trump = set_trump(deck.pop(), &dealer);
        assert_eq!(trump.rank, Rank::Wizard);
        assert_ne!(trump.suit, Suit::None);
    }

    #[test]
    fn test_calc_final_score() {
        // For now if there is a tie the player first in rotation will win.
        let players: Vec<Player> = vec![
            Player {
                score: 3,
                ..Player::new()
            },
            Player {
                score: -2,
                ..Player::new()
            },
            Player {
                score: 3,
                ..Player::new()
            },
            Player {
                score: 5,
                ..Player::new()
            },
            Player {
                score: 4,
                ..Player::new()
            },
        ];

        let winner = calc_final_score(players.clone());
        assert_eq!(players[3].score, winner.score);
    }
}
