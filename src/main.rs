use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    None,
}
impl Default for Suit {
    fn default() -> Self {
        Suit::Spade
    }
}
impl Suit {
    // Todo: Why not just make this a &str or String? The Display::fmt for this converts the char to a &str and then to a buffer...
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
        write!(f, "{}", Suit::symbol(*self).to_string())
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

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{}", self.rank.symbol(), self.suit.symbol())
    }
}

struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn build() -> Vec<Card> {
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
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

        let mut deck: Vec<Card> = Vec::new();
        for suit in suits {
            for rank in ranks {
                deck.push(Card { rank, suit });
            }
        }

        // Add 4 Wizards and Jesters..
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

    pub fn shuffle(mut cards: Vec<Card>) -> Vec<Card> {
        let deck_slice = cards.as_mut_slice();
        let mut rng = thread_rng();
        deck_slice.shuffle(&mut rng);

        cards = deck_slice.to_vec();
        cards
    }
}

enum State {
    Betting,
    Playing,
}
impl Default for State {
    fn default() -> Self {
        State::Betting
    }
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
    score: usize,
    bet: usize,
    tricks: usize,
    hand: Vec<Card>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hand = String::from("| ");
        for card in self.hand.as_slice() {
            let padded_name = format!("{}", card);
            hand.push_str(padded_name.as_str());
            hand.push_str(" | ")
        }
        write!(
            f,
            "{:8}: Score: {:>2}, Bet: {:>2}, Tricks: {:>2}, Hand: {}",
            self.name, self.score, self.bet, self.tricks, hand
        )
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
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nRound #{}: State: {}, Dealer: {}, Leader: {}, Trump: {}",
            self.round_num, self.state, self.dealer, self.leader, self.trump
        )
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
        let mut deck = Deck {
            cards: fresh_pack_of_cards.clone(),
        };
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
                if top_card.rank == Rank::Wizard {
                    // Todo: Get input from dealer to choose trump.
                    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
                    let rand_suit = rand::thread_rng().gen_range(0..3);
                    top_card.suit = suits[rand_suit];
                }
                top_card
            }
            None => Card {
                rank: Rank::None,
                suit: Suit::None,
            },
        };

        println!("{}", round);

        // Place bets.
        for i in 0..players.len() {
            round.state = State::Betting;
            // Todo: Get player input for bet.
            let max_bet = players[1].hand.len() + 1;
            players[i].bet = rand::thread_rng().gen_range(0..max_bet);
        }

        // Play Tricks.
        for _ in 1..round_num {
            round.state = State::Playing;
            let mut trick: Vec<Card> = Vec::new();

            // Play hand.
            for i in 0..players.len() {
                // Todo: Get player input for which card to play from their hand.
                // Pick random last card for now.
                let card_played = players[i].hand.pop().unwrap();
                trick.push(card_played);
            }

            let winner = calc_winner_of_trick(round.trump.suit, &trick);
            println!(
                "Winning card: {}, Player: {}",
                trick[winner], players[winner].name
            );
            players[winner].tricks = players[winner].tricks + 1;
        }

        // Calc score based off of bets and tricks.
        for i in 0..players.len() {
            if players[i].tricks == players[i].bet {
                players[i].score = players[i].score + 2;
                players[i].score = players[i].bet;
                continue;
            }
            // println!("{}", players[i]);
            // let penalty = players[i].tricks.sub() - players[i].bet;
            // println!("Penalty {}", penalty);
            // players[i].score = players[i].score + penalty;
            //
            // println!("{}", players[i]);
        }
    }
}

fn calc_winner_of_trick(trump_suit: Suit, trick: &Vec<Card>) -> usize {
    let mut winner: usize = 0;
    let mut lead_suit: Suit = trick[winner].suit;

    println!("\nTrump: {}, Lead suit: {}", trump_suit, lead_suit);

    for i in 0..trick.len() {
        println!("Played: {}, index: {}", trick[i], i);

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
                println!("new Lead suit: {}", lead_suit);
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

    println!("Final : {}, index: {}", trick[winner], winner);
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
            hand: Vec::new(),
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
    use crate::Suit;
    use crate::{calc_winner_of_trick, Card};
    use crate::{Deck, Rank};

    #[test]
    fn test_build_deck() {
        let deck = Deck::build();
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
}
