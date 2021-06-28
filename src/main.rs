//  ♠, ♥, ♣, ♦
use std::env;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit { Club, Diamond, Heart, Spade, Wizard, Jester, None}
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
            Suit::Wizard => 'W',
            Suit::Jester => 'J',
            Suit::None => 'N',
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
                suit: Suit::Wizard
            });
            deck.push(Card {
                name: String::from("Je"),
                value: 0,
                suit: Suit::Jester
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

#[derive(Clone)]
struct CardPlayed {
    card: Card,
    player: usize
}
impl fmt::Display for CardPlayed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.card, self.player)
    }
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

        // Set trump.
        round.trump = match deck.cards.pop() {
            Some(mut card) => {
                if card.suit == Suit::Wizard {
                    // Todo: Get input from dealer to choose trump.
                    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade ];
                    let rand_suit = rand::thread_rng().gen_range(0..suits.len());
                    card.suit = suits[rand_suit];
                }
                card
            },
            None => Card {
                name: String::from("No Trump"),
                value: 0,
                suit: Suit::None
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

            let winner: CardPlayed = calc_winner_of_trick(round.trump.suit, trick);
            println!("Winner: {}", winner);
        }
    }
}

fn calc_winner_of_trick(trump_suit: Suit, trick: Vec<CardPlayed>) -> CardPlayed {
    let mut winner: CardPlayed = trick[0].clone();

    let mut lead_suit: Suit = winner.card.suit;
    println!("\nTrump: {}, Lead suit: {}", trump_suit, lead_suit);

    for played in trick {
        println!("Winner: {}", winner);
        println!("Played: {} {}", played.card, played.player);

        if played.card.suit == Suit::Wizard {
            return played;
        }
        if played.card.suit == Suit::Jester {
            continue;
        }

        // If Jester was lead take suit from first non-jester.
        if winner.card.suit == Suit::Jester && played.card.suit != Suit::Jester {
            winner.card = played.card.clone();
            lead_suit = played.card.suit;
            println!("new Lead suit: {}", lead_suit);
            continue;
        }

        // If trump has already been played, compare against it.
        if winner.card.suit == trump_suit {
            if played.card.suit == trump_suit {
                if played.card.value > winner.card.value {
                    winner = played;
                    continue;
                }
            }
        }

        // Follow suit...
        if played.card.suit == lead_suit {
            if played.card.value > winner.card.value {
                winner = played;
            }
        }
    }

    winner
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


