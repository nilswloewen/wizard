use crate::components::card::Card;
use crate::components::deck::Deck;
use crate::WIZARD;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Operator {
    Human,
    Computer,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Player {
    pub name: String,
    pub score: i16,
    pub bet: u8,
    pub tricks: u8,
    pub card_played: Card,
    pub hand: Deck,
    pub operator: Operator,
    pub original_position: usize,
}
impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            score: 0,
            bet: 0,
            tricks: 0,
            card_played: WIZARD,
            hand: Deck(Vec::new()),
            operator: Operator::Computer,
            original_position: 0,
        }
    }

    pub fn print_names(players: &Vec<Player>) {
        println!("\n Players");
        println!(" -------");

        players.iter().for_each(|player| {
            println!(" {}", player.name);
        });
    }

    pub fn print_score(players: &Vec<Player>) {
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
