use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::ops;
use std::{thread, time};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    Suitless,
}
impl Suit {
    fn symbol(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
            Suit::Suitless => ' ',
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
        WIZARD
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{}", self.rank.symbol(), self.suit.symbol())
    }
}
const WIZARD: Card = Card {
    rank: Rank::Wizard,
    suit: Suit::Suitless,
};
const JESTER: Card = Card {
    rank: Rank::Jester,
    suit: Suit::Suitless,
};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Deck(pub Vec<Card>);
impl Deck {
    pub fn new() -> Deck {
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
            deck.push(WIZARD);
            deck.push(JESTER);
        }

        Deck(deck)
    }
}
impl fmt::Display for Deck {
    // Return space " " separated list of cards.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_names = String::new();
        for card in self.as_slice() {
            card_names.push_str(card.rank.symbol());
            card_names.push(card.suit.symbol());
            card_names.push_str(" ")
        }

        write!(f, "{}", card_names)
    }
}
impl ops::Deref for Deck {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

struct Cards(pub Vec<Card>);
impl ops::Deref for Cards {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Display for Cards {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| writeln!(f, "{}", card))
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Operator {
    Human,
    Computer,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Player {
    name: String,
    score: i16,
    bet: u8,
    tricks: u8,
    hand: Deck,
    operator: Operator,
    original_position: usize,
}
impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            score: 0,
            bet: 0,
            tricks: 0,
            hand: Deck::new(),
            operator: Operator::Computer,
            original_position: 0,
        }
    }

    fn print_names(players: &Vec<Player>) {
        println!("\n Players");
        println!(" -------");

        players.iter().for_each(|player| {
            println!(" {}", player.name);
        });
    }

    fn print_score(players: &Vec<Player>) {
        println!("\n Name    Score   Bet   Tricks");
        println!(" ----------------------------");
        players.iter().for_each(|player| {
            println!(
                " {:8} {:>2}     {:>2}     {:>2}",
                player.name, player.score, player.bet, player.tricks
            );
        });
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Play {
    card: Card,
    player: Player,
}
impl Play {
    fn from(card: Card) -> Play {
        Play {
            card,
            player: Player::new(),
        }
    }
}

struct VecUtil<T: fmt::Display>(Vec<T>);
impl<T: fmt::Display> fmt::Display for VecUtil<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

struct Util;
impl Util {
    fn shuffle<T: std::clone::Clone>(mut items: Vec<T>) -> Vec<T> {
        let slice = items.as_mut_slice();
        let mut rng = thread_rng();
        slice.shuffle(&mut rng);

        slice.to_vec()
    }

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

    let new_deck = Deck::new();

    let mut players = get_players();
    Player::print_names(&players);

    Util::press_enter_to_("start first round");

    // ** Short game for demo purposes. **
    // let num_rounds = new_deck.len() / players.len();
    let num_rounds = 3;

    for round_num in 1..(num_rounds + 1) {
        let mut deck = Util::shuffle(*new_deck.clone());

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
                Card::new()
            }
        };
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

    let winner = calc_winner(players);
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
            original_position: index + 1,
            ..Player::new()
        });
    }

    Util::shuffle(players)
}

fn set_trump(mut card: Card, dealer: &Player) -> Card {
    println!(" Trump: {}", card);

    if card == WIZARD {
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
    for mut player in players.iter_mut() {
        let max_bet = player.hand.len();

        match player.operator {
            Operator::Human => {
                println!("What is your bet?");

                loop {
                    player.bet = Util::cli_next_num();
                    if player.bet > max_bet as u8 {
                        println!(
                            "Yer a cocky one eh? Bet must be in the range of 0 to {}",
                            max_bet
                        );
                        continue;
                    }
                    break;
                }
            }
            Operator::Computer => {
                Util::sleep();
                player.bet = rand::thread_rng().gen_range(0..(max_bet + 1)) as u8;
            }
        };

        println!("{:>8} bet {}", player.name, player.bet);
    }
    players
}

fn play_tricks(mut players: Vec<Player>, trump: Card) -> Vec<Player> {
    for trick_num in 1..(players[0].hand.len() + 1) {
        println!("======= Trick #{} =======", trick_num);
        let mut lead_suit = Suit::Suitless;
        let mut trick: Vec<Play> = Vec::new();

        players.iter_mut().for_each(|player| {
            // Get lead suit from first non-Jester in trick.
            for played in trick.iter() {
                if lead_suit == Suit::Suitless && played.card.suit != Suit::Suitless {
                    lead_suit = played.card.suit.clone();
                    break;
                }
            }

            let played: Play = match player.operator {
                Operator::Human => play_trick_for_human(player, lead_suit),
                Operator::Computer => play_trick_for_computer(player, lead_suit),
            };

            println!("{:>8}: {}", played.player.name, played.card);
            trick.push(played);
        });

        let mut winning_play = calc_winner_of_trick(trump.suit, trick);

        winning_play.player.tricks += 1;
        println!(
            "\n  Winner: {} - {}\n========================",
            winning_play.card, winning_play.player.name
        );

        // Winner of trick should lead next trick.
        players.rotate_left(winning_play.player.original_position);

        Util::press_enter_to_("play next trick");
    }

    players
}

fn play_trick_for_human(player: &mut Player, lead_suit: Suit) -> Play {
    let mut can_follow_suit = false;

    println!("\nYour hand:");
    for (index, card) in player.hand.iter().enumerate() {
        println!("  {}. {}", index + 1, card);
        if !can_follow_suit {
            if card.suit == lead_suit {
                // Not Suit::Suitless is needed because lead_suit is initialized
                // as Suit::Suitless and it's possible a Wizard or Jester could match here.
                can_follow_suit = card.suit != Suit::Suitless;
                break;
            }
        }
    }

    println!("Which card will you play?");

    loop {
        let selection = Util::cli_next_pos_num() as usize - 1;

        if selection >= player.hand.len() {
            println!("Hey! Gotta pick what's offered!");
            continue;
        }

        let card = player
            .hand
            .drain(selection..(selection + 1))
            .last()
            .unwrap();

        if can_follow_suit {
            if card.suit != lead_suit {
                println!("Hey! Gotta follow suit!");
                player.hand.insert(selection, card);
                continue;
            }
        }

        return Play {
            card,
            player: player.clone(),
        };
    }
}

fn play_trick_for_computer(player: &mut Player, lead_suit: Suit) -> Play {
    Util::sleep();

    // Play first card in hand if suit cannot be followed.
    let mut selected = 0;
    for (index, card) in player.hand.iter().enumerate() {
        if card.suit == lead_suit {
            selected = index;
            break;
        }
    }

    Play {
        card: player.hand.drain(selected..(selected + 1)).last().unwrap(),
        player: player.clone(),
    }
}

fn calc_winner_of_trick(trump_suit: Suit, mut trick: Vec<Play>) -> Play {
    let first_card = trick.drain(0..1).last().unwrap();
    let mut lead_suit = first_card.card.suit;
    let mut winning = first_card;

    for current in trick {
        if current.card == WIZARD {
            return current;
        }
        if current.card == JESTER {
            continue;
        }

        // If Jester was led take suit from first non-Jester.
        if winning.card == JESTER {
            if current.card != JESTER {
                winning = current;
                lead_suit = winning.card.suit;
                continue;
            }
        }

        if winning.card.suit == trump_suit {
            if current.card.suit == trump_suit {
                if current.card.rank.value() > winning.card.rank.value() {
                    winning = current;
                    continue;
                }
            }
            continue;
        }

        if current.card.suit == trump_suit {
            winning = current;
            continue;
        }

        // Follow suit...
        if current.card.suit == lead_suit {
            if current.card.rank.value() > winning.card.rank.value() {
                winning = current;
            }
        }
    }

    winning
}

fn calc_score(mut players: Vec<Player>) -> Vec<Player> {
    for mut player in &mut players {
        if player.tricks == player.bet {
            player.score += (2 + player.bet) as i16;
            continue;
        }

        let penalty = player.bet as i16 - player.tricks as i16;
        player.score -= penalty.abs();
    }
    players
}

fn calc_winner(mut players: Vec<Player>) -> Player {
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
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_build_deck() {
        let deck = Deck::new();
        assert_eq!(60, deck.len());
    }

    #[test]
    fn test_calc_trick() {
        let mut trick = vec![
            Play::from(Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            }),
            Play::from(Card {
                rank: Rank::King,
                suit: Suit::Heart,
            }),
            Play::from(Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            }),
        ];

        // Test all non-trump, no Wizard or Jester.
        assert_eq!(trick[1], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // Ace of lead suit should now win.
        trick.push(Play::from(Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        }));
        assert_eq!(trick[3], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // Low Trump should now win.
        trick.push(Play::from(Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        }));
        assert_eq!(trick[4], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // Higher Trump should now win.
        trick.push(Play::from(Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        }));
        assert_eq!(trick[5], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // First Wizard always wins.
        trick.clear();
        for _ in 0..3 {
            trick.push(Play::from(WIZARD));
        }
        assert_eq!(trick[0], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // First Jester wins if all Jesters.
        trick.clear();
        for _ in 0..3 {
            trick.push(Play::from(JESTER));
        }
        assert_eq!(trick[0], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // First non-Jester sets lead suit.
        trick.push(Play::from(Card {
            rank: Rank::Two,
            suit: Suit::Diamond,
        }));
        assert_eq!(trick[3], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // New lead suit is now followed.
        trick.push(Play::from(Card {
            rank: Rank::Ace,
            suit: Suit::Diamond,
        }));
        assert_eq!(trick[4], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // Trump still wins.
        trick.push(Play::from(Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        }));
        assert_eq!(trick[5], calc_winner_of_trick(Suit::Spade, trick.clone()));

        // If there is no trump then highest lead suit wins.
        assert_eq!(
            trick[4],
            calc_winner_of_trick(Suit::Suitless, trick.clone())
        );

        // Make sure second Jester doesn't mess up lead suit.
        trick = vec![
            Play::from(JESTER),
            Play::from(Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            }),
            Play::from(JESTER),
            Play::from(Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            }),
            Play::from(Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }),
            Play::from(Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }),
        ];

        assert_eq!(trick[5], calc_winner_of_trick(Suit::Diamond, trick.clone()));
    }

    #[test]
    fn test_calc_score() {
        let mut players: Vec<Player> = Vec::new();
        let mut player = Player::new();

        // First player bets 0 and gets 0 tricks, should have score of 3.
        players.push(player.clone());

        // Second player bets 1 and gets 0 tricks, should have score of -1.
        player.bet = 1;
        players.push(player.clone());

        // Third player bets 1 and gets 2 tricks, should have score of -1.
        player.tricks = 2;
        players.push(player.clone());

        players = calc_score(players);
        assert_eq!(players[0].score, 2);
        assert_eq!(players[1].score, -1);
        assert_eq!(players[2].score, -1);
    }

    #[test]
    fn test_set_trump() {
        let dealer = Player::new();

        // Normal card should be returned as trump.
        let two_of_hearts = Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        };
        let mut trump = set_trump(two_of_hearts, &dealer);
        assert_eq!(trump, two_of_hearts);

        // Nothing special happens for Jester, returned like normal card.
        trump = set_trump(JESTER, &dealer);
        assert_eq!(trump, JESTER);

        // If Wizard is flipped the dealer should choose a suit for trump.
        trump = set_trump(WIZARD, &dealer);
        assert_eq!(trump.rank, Rank::Wizard);
        assert_ne!(trump.suit, Suit::Suitless);
    }

    #[test]
    fn test_calc_final_score() {
        let mut players: Vec<Player> = Vec::new();
        for _ in 0..6 {
            players.push(Player::new());
        }

        players[0].score = 3;
        players[1].score = -2;
        players[2].score = 3;
        players[3].score = 5;
        players[4].score = 4;
        players[5].score = 0;

        // For now if there is a tie the player first in rotation will win.
        let winner = calc_winner(players.clone());
        assert_eq!(players[3].score, winner.score);
    }
}
