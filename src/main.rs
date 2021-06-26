//  ♠, ♥, ♣, ♦
use std::env;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit { Club, Diamond, Heart, Spade, Special }
impl Default for Suit {
    fn default() -> Self { Suit::Spade}
}
impl Suit {
    // Todo: Why not just make this a &str or String? The Display::fmt for this converts the char to a &str and then to a buffer...
    fn char(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
            Suit::Special => '~',
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Suit::char(*self).to_string())
    }
}

#[derive(Clone, Default)]
struct Card {
    name: String,
    value: usize,
    suit: Suit
}
impl Card {
    fn default() -> Self {
        Card {name: String::from("A"), value: 14, suit: Suit::Spade }
    }
    pub fn print(&self) {
        println!("{:>2} {:>6} {}", self.value, self.name, self.suit);
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{}", self.name.as_str(), self.suit.char())
    }
}

struct Deck {
    cards: Vec<Card>
}
impl Deck {
    pub fn build() -> Vec<Card> {
        // Build normal 52 card deck.
        let mut deck: Vec<Card> = Vec::new();
        let names = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade ];
        for suit in suits {
            for (index, name) in names.iter().enumerate() {
                deck.push( Card {
                    value: index + 2,
                    name: String::from(name.clone()),
                    suit
                });
            }
        }

        // Add 4 Wizards and Jesters..
        for _ in 0..4 {
            deck.push(Card {
                name: String::from("W"),
                value: 15,
                suit: Suit::Special
            });
            deck.push(Card {
                name: String::from("Je"),
                value: 0,
                suit: Suit::Special
            });
        }

        deck
    }

    pub fn shuffle(mut cards: Vec<Card>) -> Vec<Card> {
        let deck_slice = cards.as_mut_slice();
        let mut rng = thread_rng();
        deck_slice.shuffle(&mut rng);

        cards = deck_slice.to_vec();
        cards
    }

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }
}

enum State { Betting, Playing }
impl Default for State {
    fn default() -> Self { State::Betting }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Betting => write!(f, "Betting"),
            State::Playing => write!(f, "Playing"),
        }
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    score: u8,
    bet: usize,
    tricks: u8,
    hand: Vec<Card>
}
impl Player {
    pub fn print(&self) {
        let mut hand = String::from("| ");
        for card in self.hand.as_slice() {
            let padded_name = format!("{}", card);
            hand.push_str(padded_name.as_str());
            hand.push_str(" | ")
        }
        println!("{:8}: Score: {:>2}, Bet: {:>2}, Tricks: {:>2}, Hand: {}", self.name, self.score, self.bet, self.tricks, hand)
    }
}

#[derive(Default)]
struct Round {
    state: State,
    round_num: usize,
    // Todo: Make reference to Player object.
    dealer: String,
    leader: String,
    trump: Card,
}
impl Round {
    pub fn print(&self) {
        println!("\nRound #{}: State: {}, Dealer: {}, Leader: {}, Trump: {}", self.round_num, self.state, self.dealer, self.leader, self.trump)
    }
    fn default() -> Self {
        Round {
            state: State::Betting,
            round_num: 0,
            dealer: String::new(),
            leader: String::new(),
            trump:  Card::default()
        }
    }
}

struct CardPlayed {
    card: Card,
    player: usize
}

fn main() {
    print_wizard_ascii_art();

    let starting_players = get_players();
    let fresh_pack_of_cards = Deck::build();
    let mut round: Round = Default::default();
    let num_rounds = 60 / starting_players.len();

    for round_num in 1..(num_rounds + 1) {
        round.round_num = round_num;

        // Get players and rotate dealer.
        let mut players = starting_players.clone();
        let player_rotation = round_num % &players.len();
        players.rotate_left(player_rotation);
        round.dealer = String::from(&players[0].name);

        // Rotate again so leader gets first deal and starts betting round.
        players.rotate_left(1);
        round.leader = String::from(&players[0].name);

        // Shuffle new deck.
        let mut deck = Deck { cards: fresh_pack_of_cards.clone() };
        deck.cards = Deck::shuffle(deck.cards);

        // Deal
        for i in 0..players.len() {
            for _ in 0..round_num {
                players[i].hand.push(deck.cards.pop().unwrap());
            }
        }

        round.trump = match deck.cards.pop() {
            Some(card) => card,
            None => Card {
                name: String::from("No Trump"),
                value: 0,
                suit: Suit::Special
            }
        };

        // Place random bets for now.
        for i in 0..players.len() {
            let max_bet = players[1].hand.len() + 1;
            players[i].bet = rand::thread_rng().gen_range(0..max_bet);
        }

        // Play Tricks.
        for _ in 1..round_num {
            let mut trick: Vec<CardPlayed> = Vec::new();

            // Play hand.
            for i in 0..players.len() {
                // Pick random last card for now.
                let card_played = players[i].hand.pop().unwrap();
                trick.push(CardPlayed {
                    card: card_played,
                    player: i
                });
            }

            // Calculate winner.
            let mut winning_card = trick[0].card.clone();
            let mut winning_player = trick[0].player;

            // Break after first wizard is counted.
            if winning_card.name.as_str() == "W" {
                println!("Winning card: {}, player {}", winning_card, winning_player);
            }
            else {
                let mut lead_suit: Suit = winning_card.suit.clone();
                println!("\nLead suit: {}", lead_suit);

                for played in trick {
                    println!("Played: {} {}", played.card, played.player);
                    // If Jester is lead take suit from first non-jester.
                    if winning_card.name.as_str() == "Je" && played.card.name.as_str() != "Je" {
                        winning_card = played.card.clone();
                        lead_suit = played.card.suit.clone();
                        println!("new Lead suit: {}", lead_suit);
                        continue;
                    }

                    // Break after first wizard is counted.
                    if played.card.name.as_str() == "W" {
                        winning_card = played.card.clone();
                        winning_player = played.player.clone();
                        break;
                    }

                    // Follow suit...
                    if played.card.suit == lead_suit {
                        if played.card.value > winning_card.value {
                            winning_card = played.card.clone();
                            winning_player = played.player.clone();
                        }
                    }
                }
                println!("Winning card: {}, player {}", winning_card, winning_player);
            }
        }
    }
}


fn bet(mut player: Player) -> Player {
    player
}

fn get_players() -> Vec<Player> {
    let mut args: Vec<String> = env::args().collect();
    // Drop the first arg as it is the command name.
    args.drain(0..1);

    let mut players: Vec<Player> = Vec::new();
    for name in args {
        players.push(Player {
            name: name.clone(),
            score: 0,
            bet: 0,
            tricks: 0,
            hand: Vec::new()
        })
    }

    // Rotate back so that the first arg ends up as the first dealer.
    players.rotate_right(1);
    players
}

fn print_wizard_ascii_art() {
    println!("           _                  _\n          (_)                | |\n __      ___ ______ _ _ __ __| |\n \\ \\ /\\ / / |_  / _` | \'__/ _` |\n  \\ V  V /| |/ / (_| | | | (_| |\n   \\_/\\_/ |_/___\\__,_|_|  \\__,_|\n");
}


