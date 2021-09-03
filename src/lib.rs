pub mod components;

use crate::components::card::*;
use crate::components::player::*;
use crate::components::util::Util;
use rand::Rng;

pub const WIZARD: Card = Card {
    rank: Rank::Wizard,
    suit: Suit::Suitless,
};
pub const JESTER: Card = Card {
    rank: Rank::Jester,
    suit: Suit::Suitless,
};

pub fn get_players() -> Vec<Player> {
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

    Util::shuffle_vec(players)
}

pub fn set_trump(mut card: Card, dealer: &Player) -> Card {
    println!(" Trump: {}", card);

    if card == WIZARD {
        println!("\nTrump is a Wizard!");
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

        match dealer.operator {
            Operator::Human => {
                println!("Which suit do you select as trump?");
                for i in 0..suits.len() {
                    println!("  {}. {}", i + 1, suits[i].symbol());
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

        println!("\n Trump suit: {}", card.suit.symbol());
    }
    card
}

#[test]
pub fn test_set_trump() {
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

pub fn place_bets(players: &mut Vec<Player>) {
    for player in players.iter_mut() {
        let max_bet = player.hand.len();

        match player.operator {
            Operator::Human => {
                println!("What is your bet?");

                loop {
                    player.bet = Util::cli_next_num();
                    if player.bet > max_bet as u8 {
                        println!(
                            "Yer a cocky one eh?\nBet must be in the range of 0 to {}.",
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
}

pub fn play_tricks(mut players: &mut Vec<Player>, trump: Card) {
    for trick_num in 1..(players[0].hand.len() + 1) {
        println!("======= Trick #{} =======", trick_num);
        let mut lead_suit = Suit::Suitless;

        for player in players.clone() {
            if player.card_played != JESTER {
                lead_suit = player.card_played.suit.clone();
                break;
            }
        }

        players.iter_mut().for_each(|mut player: &mut Player| {
            match player.operator {
                Operator::Human => get_play_from_human(&mut player, lead_suit),
                Operator::Computer => get_play_from_comp(&mut player, lead_suit),
            };

            println!("{:>8}: {}", player.name, player.card_played);
        });

        calc_winner_of_trick(&mut players, trump.suit);

        Util::press_enter_to_("play next trick");
    }
}

pub fn get_play_from_human(player: &mut Player, lead_suit: Suit) {
    println!("\nYour hand:");
    for (index, card) in player.hand.iter().enumerate() {
        println!("  {}. {}", index + 1, card);
    }

    let mut can_follow_suit = false;

    // Not Suit::Suitless is needed because lead_suit is initialized as Suit::Suitless and it's possible a Wizard or Jester could match here.
    if lead_suit != Suit::Suitless {
        for card in player.hand.iter() {
            if card.suit == lead_suit {
                can_follow_suit = true;
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

        player.card_played = card;
        return;
    }
}

pub fn get_play_from_comp(player: &mut Player, lead_suit: Suit) {
    Util::sleep();

    // Play first card in hand if suit cannot be followed.
    let mut selected = 0;
    for (index, card) in player.hand.iter().enumerate() {
        if card.suit == lead_suit {
            selected = index;
            break;
        }
    }

    let card = player.hand.drain(selected..(selected + 1)).last().unwrap();
    player.card_played = card;
}

pub struct Play {
    card: Card,
    player_index: usize,
}

pub fn calc_winner_of_trick(players: &mut Vec<Player>, trump_suit: Suit) {
    let mut winning = Play {
        card: players[0].card_played.clone(),
        player_index: 0,
    };
    let mut lead_suit = winning.card.suit;

    for (index, current) in players.iter().enumerate() {
        if current.card_played == WIZARD {
            winning.card = current.card_played.clone();
            winning.player_index = index;
            break;
        }

        if current.card_played == JESTER {
            continue;
        }

        // If Jester was led take suit from first non-Jester.
        if winning.card == JESTER {
            if current.card_played != JESTER {
                winning.card = current.card_played.clone();
                winning.player_index = index;
                lead_suit = winning.card.suit;
                continue;
            }
        }

        if winning.card.suit == trump_suit {
            if current.card_played.suit == trump_suit {
                if current.card_played.rank.value() > winning.card.rank.value() {
                    winning.card = current.card_played.clone();
                    winning.player_index = index;
                    continue;
                }
            }
            continue;
        }

        if current.card_played.suit == trump_suit {
            winning.card = current.card_played.clone();
            winning.player_index = index;
            continue;
        }

        // Follow suit...
        if current.card_played.suit == lead_suit {
            if current.card_played.rank.value() > winning.card.rank.value() {
                winning.card = current.card_played.clone();
                winning.player_index = index;
            }
        }
    }

    players[winning.player_index].tricks += 1;
    let winner = players[winning.player_index].clone();
    println!(
        "\n  Winner: {} - {}\n========================",
        winner.card_played, winner.name
    );

    // Winner of trick should lead next trick.
    players.rotate_left(winner.original_position);
}

#[test]
pub fn test_calc_trick() {
    // Test all non-trump, no Wizard or Jester.
    let mut players = vec![
        Player {
            card_played: Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            ..Player::new()
        },
    ];

    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[1].tricks, 1);

    // Ace of lead suit should now win.
    players.push(Player {
        card_played: Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[3].tricks, 1);

    // Low trump should now win.
    players.push(Player {
        card_played: Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[4].tricks, 1);

    // Higher Trump should now win.
    players.push(Player {
        card_played: Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[5].tricks, 1);

    // First Wizard always wins.
    players.clear();
    for _ in 0..3 {
        players.push(Player {
            card_played: WIZARD,
            ..Player::new()
        });
    }
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[0].tricks, 1);

    // First Jester wins if all Jesters.
    players.clear();
    for _ in 0..3 {
        players.push(Player {
            card_played: JESTER,
            ..Player::new()
        });
    }
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[0].tricks, 1);

    // First non-Jester sets lead suit.
    players.push(Player {
        card_played: Card {
            rank: Rank::Two,
            suit: Suit::Diamond,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[3].tricks, 1);

    // New lead suit is now followed.
    players.push(Player {
        card_played: Card {
            rank: Rank::Ace,
            suit: Suit::Diamond,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[4].tricks, 1);

    // Trump still wins.
    players.push(Player {
        card_played: Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        },
        ..Player::new()
    });
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[5].tricks, 1);

    // If there is no trump then highest lead suit wins.
    calc_winner_of_trick(&mut players, Suit::Suitless);
    assert_eq!(players[4].tricks, 2);

    // Make sure second Jester doesn't mess up lead suit.
    players = vec![
        Player {
            card_played: JESTER,
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            ..Player::new()
        },
        Player {
            card_played: JESTER,
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            ..Player::new()
        },
        Player {
            card_played: Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            ..Player::new()
        },
    ];
    calc_winner_of_trick(&mut players, Suit::Spade);
    assert_eq!(players[5].tricks, 1);
}

pub fn calc_score(players: &mut Vec<Player>) {
    for mut player in players {
        if player.tricks == player.bet {
            player.score += (2 + player.bet) as i16;
            continue;
        }

        let penalty = player.bet as i16 - player.tricks as i16;
        player.score -= penalty.abs();
    }
}

#[test]
pub fn test_calc_score() {
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

    calc_score(&mut players);
    assert_eq!(players[0].score, 2);
    assert_eq!(players[1].score, -1);
    assert_eq!(players[2].score, -1);
}

pub fn calc_winner(mut players: Vec<Player>) -> Player {
    let mut winner = players.pop().unwrap();
    for player in players {
        if player.score > winner.score {
            winner = player;
        }
    }
    winner
}

#[test]
pub fn test_calc_winner() {
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
