use std::env;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::panic::panic_any;
use crate::State::Playing;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Suit {Club, Diamond, Heart, Spade, Wizard, Jester, None}
impl Default for Suit {
    fn default() -> Self {Suit::Spade}
}
impl Suit {
    // Todo: Why not just make this a &str or String? The Display::fmt for this converts the char to a &str and then to a buffer...
    fn symbol(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
            Suit::Wizard => 'W',
            Suit::Jester => 'J',
            Suit::None => ' ',
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Suit::symbol(*self).to_string())
    }
}

#[derive(Clone, Copy)]
enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace, Wizard, Jester, None
}
impl Default for Rank {
    fn default() -> Self { Rank::None }
}
impl Rank {
    fn symbol(self) -> &'static str {
        match self {
            Rank::Two => "2", Rank::Three => "3", Rank::Four => "4",
            Rank::Five => "5", Rank::Six => "6", Rank::Seven => "7",
            Rank::Eight => "8", Rank::Nine => "9", Rank::Ten => "10",
            Rank::Jack => "J", Rank::Queen => "Q", Rank::King => "K",
            Rank::Ace => "A", Rank::Wizard => "W", Rank::Jester => "Je",
            Rank::None => ""
        }
    }
    fn rank (self) -> u8 {
        match self {
            Rank::Two => 2, Rank::Three => 3, Rank::Four => 4,
            Rank::Five => 5, Rank::Six => 6, Rank::Seven => 7,
            Rank::Eight => 8, Rank::Nine => 9, Rank::Ten => 10,
            Rank::Jack => 11, Rank::Queen => 12, Rank::King => 13,
            Rank::Ace => 14, Rank::Wizard => 15, Rank::Jester => 0,
            Rank::None => 0
        }
    }
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}", Rank::symbol(*self))
    }
}

#[derive(Clone, Default)]
struct Card {
    face: Rank,
    suit: Suit
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{}", self.face.symbol(), self.suit.symbol())
    }
}

struct Deck {
    cards: Vec<Card>
}
impl Deck {
    pub fn build() -> Vec<Card> {
        // Build normal 52 card deck.
        let faces = [ Rank::Ace,
            Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine,
            Rank::Ten, Rank::Jack, Rank::Queen, Rank::King,
        ];
        let suits = [ Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade ];

        let mut deck: Vec<Card> = Vec::new();
        for suit in suits {
            for face in faces {
                deck.push( Card { face, suit });
            }
        }

        // Add 4 Wizards and Jesters..
        for face in [ Rank::Wizard, Rank::Jester ] {
            deck.push(Card { face, suit: Suit::None });
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
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hand = String::from("| ");
        for card in self.hand.as_slice() {
            let padded_name = format!("{}", card);
            hand.push_str(padded_name.as_str());
            hand.push_str(" | ")
        }
        write!(f, "{:8}: Score: {:>2}, Bet: {:>2}, Tricks: {:>2}, Hand: {}", self.name, self.score, self.bet, self.tricks, hand)
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
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nRound #{}: State: {}, Dealer: {}, Leader: {}, Trump: {}", self.round_num, self.state, self.dealer, self.leader, self.trump)
    }
}

#[derive(Clone)]
struct CardPlayed {
    card: Card,
    player_num: usize,
    player: Player
}
impl fmt::Display for CardPlayed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.card, self.player_num, self.player)
    }
}

fn main() {
    print_wizard_ascii_art();

    let starting_players = get_players();
    let fresh_pack_of_cards = Deck::build();
    let mut round: Round = Default::default();
    let num_rounds = fresh_pack_of_cards.len() / starting_players.len();

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
            Some(mut top_card) => {
                if top_card.suit == Suit::Wizard {
                    // Todo: Get input from dealer to choose trump.
                    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
                    let rand_suit = rand::thread_rng().gen_range(0..3);
                    top_card.suit = suits[rand_suit];
                }
                if top_card.suit == Suit::Jester {
                    top_card.suit = Suit::None;
                }
                top_card
            }
            None => Card {
                face: Rank::None,
                suit: Suit::None
            }
        };

        println!("{}", round);
        // Place bets.
        for i in 0..players.len() {
            // Todo: Get player input for bet.
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
                    player_num: i,
                    player: players.pop().unwrap()
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
        println!("Played: {}, {}", played.card, played.player_num);

        if played.card.suit == Suit::Wizard {
            return played;
        }
        if played.card.suit == Suit::Jester {
            continue;
        }

        // If Jester was led take suit from first non-jester.
        if winner.card.suit == Suit::Jester {
            if played.card.suit != Suit::Jester {
                winner.card = played.card.clone();
                lead_suit = played.card.suit;
                println!("new Lead suit: {}", lead_suit);
                continue;
            }
        }

        // If trump has already been played, compare against it.
        if winner.card.suit == trump_suit {
            if played.card.suit == trump_suit {
                if  played.card.face.rank() > winner.card.face.rank() {
                    winner = played;
                    continue;
                }
            }
        }

        // Follow suit...
        if played.card.suit == lead_suit {
            if played.card.face.rank() > winner.card.face.rank() {
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

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}

