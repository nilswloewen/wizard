use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::io;

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
impl Default for Rank {
    fn default() -> Self {
        Rank::None
    }
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hand = String::new();
        for card in self.cards.as_slice() {
            let padded_name = format!("|{}|", card);
            hand.push_str(padded_name.as_str());
            hand.push_str(" ")
        }

        write!(f, "{}", hand)
    }
}

#[derive(Clone, Eq, PartialEq)]
enum PlayerType {
    Human,
    Computer,
}

#[derive(Clone)]
struct Player {
    name: String,
    score: i8,
    bet: i8,
    tricks: i8,
    hand: Deck,
    kind: PlayerType,
}
impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            score: 0,
            bet: 0,
            tricks: 0,
            hand: Deck { cards: Vec::new() },
            kind: PlayerType::Computer,
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
        for i in 0..players.len() {
            println!(" {}", players[i].name);
        }
    }
    fn print_score(players: &Vec<Player>) {
        println!();
        println!(" Name    Score");
        println!(" -------------");
        for i in 0..players.len() {
            println!(" {:8} {:>2}", players[i].name, players[i].score);
        }
    }
    fn print_score_bet(players: &Vec<Player>) {
        println!();
        println!(" Name    Score   Bet   Tricks");
        println!(" ----------------------------");
        for i in 0..players.len() {
            println!(
                " {:8} {:>2}     {:>2}     {:>2}",
                players[i].name, players[i].score, players[i].bet, players[i].tricks
            );
        }
        println!();
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>8}: Score: {:>3}, Bet: {:>2}, Tricks: {:>2}, Hand: {}",
            self.name, self.score, self.bet, self.tricks, self.hand
        )
    }
}

struct Round {
    round_num: usize,
    dealer: Player,
    leader: Player,
    trump: Card,
}
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n--- Round #{} --- \nDealer: {} \nLeader: {} \n Trump: {}",
            self.round_num, self.dealer.name, self.leader.name, self.trump
        )
    }
}

fn main() {
    print_wizard_ascii_art();

    let original_players = get_players();
    let mut players = original_players.clone();
    let fresh_pack_of_cards = Deck { cards: Deck::new() };
    let mut round = Round {
        round_num: 0,
        dealer: players[0].clone(),
        leader: players[1].clone(),
        trump: Card {
            rank: Rank::None,
            suit: Suit::None,
        },
    };
    let num_rounds = (fresh_pack_of_cards.cards.len() / players.len()) - 5;

    for round_num in 1..(num_rounds + 1) {
        round.round_num = round_num;

        // Get players and rotate dealer.
        let player_rotation = round_num % &players.len();
        players.rotate_left(player_rotation);
        round.dealer = players[0].clone();

        // Rotate again so leader gets first deal and starts betting round.
        players.rotate_left(1);
        round.leader = players[0].clone();

        // Shuffle new deck.
        let mut deck = Deck::shuffle(fresh_pack_of_cards.cards.clone());

        // Deal
        for i in 0..players.len() {
            players[i].hand.cards.clear();
            for _ in 0..round_num {
                players[i].hand.cards.push(deck.pop().unwrap());
            }
        }

        round.trump = set_trump(deck.pop());

        println!("{}\n----------------", round);
        press_enter_to_("start betting");

        players = place_bets(players);
        press_enter_to_("play first trick");

        players = play_tricks(players, &round);
        players = calc_score(players);

        // Reset order
        let mut players_in_order = players[0].name == original_players[1].name;
        while !players_in_order {
            players.rotate_left(1);
            players_in_order = players[0].name == original_players[1].name;
        }

        println!("End of round #{} - Current standings:", round.round_num);
        Player::print_score_bet(&players);

        press_enter_to_("start next round");
    }
}

fn print_wizard_ascii_art() {
    println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
}

fn get_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();

    // Get name from cli.
    println!("Enter your name:");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name = String::from(name_input.trim());

    players.push(Player {
        name,
        kind: PlayerType::Human,
        ..Player::new()
    });
    let computer_names = ["Alice", "Bob", "Chuck", "Daniels", "Eli"];
    for name in computer_names {
        players.push(Player {
            name: String::from(name),
            kind: PlayerType::Computer,
            ..Player::new()
        });
    }

    players = Player::shuffle(players);

    Player::print_names(&players);
    press_enter_to_("continue");

    // Rotate back so that the first arg ends up as the first dealer when rotated later on.
    players.rotate_right(1);
    players
}

fn press_enter_to_(mut verb: &str) {
    println!("\nPress Enter to {}...", verb);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
}

fn set_trump(top_card: Option<Card>) -> Card {
    match top_card {
        Some(mut card) => {
            if card.rank == Rank::Wizard {
                // Todo: Get input from dealer to choose trump.
                let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
                let rand_suit = rand::thread_rng().gen_range(0..3);
                card.suit = suits[rand_suit];
            }
            card
        }
        None => Card {
            rank: Rank::None,
            suit: Suit::None,
        },
    }
}

fn place_bets(mut players: Vec<Player>) -> Vec<Player> {
    // Place bets.
    for i in 0..players.len() {
        if players[i].kind == PlayerType::Human {
            let mut hand = String::new();
            for j in 0..players[i].hand.cards.len() {
                hand.push_str(format!("|{}| ", players[i].hand.cards[j]).as_str());
            }
            println!("\nYour hand: {}", hand);
            println!("\nWhat is your bet?");

            let mut bet_input = String::new();
            io::stdin().read_line(&mut bet_input).unwrap();
            players[i].bet = bet_input.trim().parse().unwrap();
            continue;
        }

        // Computer players place random bets for now.
        let max_bet = players[1].hand.cards.len() + 1;
        players[i].bet = rand::thread_rng().gen_range(0..max_bet) as i8;
        println!("{:>8} bet {}", players[i].name, players[i].bet);
    }

    players
}

fn play_tricks(mut players: Vec<Player>, round: &Round) -> Vec<Player> {
    // Reset tricks.
    for i in 0..players.len() {
        players[i].tricks = 0;
    }

    // Play Tricks.
    for i in 1..(round.round_num + 1) {
        println!("=== Trick #{} ===", i);
        let mut trick: Vec<Card> = Vec::new();

        // Play hand.
        for i in 0..players.len() {
            match players[i].kind {
                PlayerType::Human => {
                    println!("\nYour hand:");
                    for j in 0..players[i].hand.cards.len() {
                        println!("  {}. | {}|", j + 1, players[i].hand.cards[j]);
                    }

                    println!("Which card will you play?");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let mut play: usize = input.trim().parse().unwrap();
                    play -= 1;

                    trick.push(players[i].hand.cards.drain(play..play + 1).last().unwrap());
                    continue;
                },
                PlayerType::Computer => {

                    // Pick random last card for now.
                    let card_played = players[i].hand.cards.pop().unwrap();
                    trick.push(card_played.clone());
                    println!("{:>8}: {}", players[i].name, card_played);
                }
            }
        }

        let winner = calc_winner_of_trick(round.trump.suit, &trick);
        println!("  Winner: {} - {}", trick[winner], players[winner].name);
        players[winner].tricks += 1;

        // Winner of trick should lead next trick.
        players.rotate_left(winner);

        press_enter_to_("play next trick");
    }

    players
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

        if trick[i].suit == trump_suit {
            if trick[winner].suit == trump_suit {
                if trick[i].rank.value() > trick[winner].rank.value() {
                    winner = i;
                    continue;
                }
            }

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
    for i in 0..players.len() {
        if players[i].tricks == players[i].bet {
            players[i].score += 2 + players[i].bet;
            continue;
        }

        let penalty: i8 = (players[i].bet - players[i].tricks).abs();
        players[i].score -= penalty;
    }
    players
}

#[cfg(test)]
mod tests {
    use crate::{calc_score, set_trump, Player, Suit};
    use crate::{calc_winner_of_trick, Card};
    use crate::{Deck, Rank};

    #[test]
    fn test_build_deck() {
        let deck = Deck::new();
        assert_eq!(60, deck.len());
    }

    #[test]
    fn test_calc_trick() {
        let trump = Suit::Spade;
        let mut trick: Vec<Card> = Vec::new();

        // Test all non-trump, no special.
        trick.push(Card {
            rank: Rank::Queen,
            suit: Suit::Heart,
        });
        trick.push(Card {
            rank: Rank::King,
            suit: Suit::Heart,
        });
        trick.push(Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        });
        assert_eq!(1, calc_winner_of_trick(trump, &trick));

        // Ace of lead should now win.
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

        // Wizard killz.
        trick.push(Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        });
        assert_eq!(6, calc_winner_of_trick(trump, &trick));

        // First Wizard always wins.
        let mut trick: Vec<Card> = Vec::new();
        trick.push(Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        });
        trick.push(Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        });
        trick.push(Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        });
        assert_eq!(0, calc_winner_of_trick(trump, &trick));

        // First Jester wins if all Jesters.
        let mut trick: Vec<Card> = Vec::new();
        trick.push(Card {
            rank: Rank::Jester,
            suit: Suit::None,
        });
        trick.push(Card {
            rank: Rank::Jester,
            suit: Suit::None,
        });
        trick.push(Card {
            rank: Rank::Jester,
            suit: Suit::None,
        });
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
    }

    #[test]
    fn test_calc_score() {
        let mut players: Vec<Player> = Vec::new();
        let mut player = Player {
            name: String::from("Steve"),
            score: 0,
            bet: 0,
            tricks: 0,
            hand: Deck { cards: Vec::new() },
        };
        players.push(player.clone());
        player.bet = 1;
        players.push(player.clone());

        player.tricks = 2;
        players.push(player.clone());

        players = calc_score(players);
        assert_eq!(players[0].score, 2);
        assert_eq!(players[1].score, -1);
        assert_eq!(players[2].score, -1);
    }

    #[test]
    fn test_set_trump() {
        // On last round deck will be empty, trump should be null card.
        let mut deck: Vec<Card> = Vec::new();
        let mut trump = set_trump(deck.pop());
        assert_eq!(
            trump,
            Card {
                rank: Rank::None,
                suit: Suit::None
            }
        );

        // Normal card should be returned as trump.
        let two_of_hearts = Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        };
        deck.push(two_of_hearts.clone());
        trump = set_trump(deck.pop());
        assert_eq!(trump, two_of_hearts);

        // Nothing special happens for Jester, returned like normal card.
        let jester = Card {
            rank: Rank::Jester,
            suit: Suit::None,
        };
        deck.push(jester.clone());
        trump = set_trump(deck.pop());
        assert_eq!(trump, jester);

        // If Wizard is flipped the dealer should choose a suit for trump.
        let wizard = Card {
            rank: Rank::Wizard,
            suit: Suit::None,
        };
        deck.push(wizard);
        trump = set_trump(deck.pop());
        assert_eq!(trump.rank, Rank::Wizard);
        assert_ne!(trump.suit, Suit::None);
    }
}
