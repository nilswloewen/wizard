use crate::components::card::{Card, Rank, Suit};
use crate::{JESTER, WIZARD};
use core::{fmt, ops};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Deck(pub Vec<Card>);
impl Deck {
    pub fn build() -> Deck {
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
        self.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| write!(f, "{} ", card))
        })
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

#[test]
pub fn test_build_deck() {
    let deck = Deck::build();
    assert_eq!(60, deck.len());
}
