const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_PONTS: i32 = 3;

const WIN_POINTS: i32 = 6;
const LOSE_POINTS: i32 = 0;
const TIE_POINTS: i32 = 3;

use Choice::*;
use Outcome::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Choice {
    fn score(self: &Choice) -> i32 {
        match self {
            Rock => ROCK_POINTS,
            Paper => PAPER_POINTS,
            Scissors => SCISSORS_PONTS,
        }
    }

    pub fn beats(self: Choice) -> Choice {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn loses_to(self: Choice) -> Choice {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    pub fn fix_game(self: Choice, outcome: Outcome) -> Choice {
        match outcome {
            Win => self.loses_to(),
            Lose => self.beats(),
            Tie => self
        }
    }
}

#[readonly::make]
pub struct Game {
    pub p1_score: i32,
    pub p2_score: i32,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            p1_score: 0,
            p2_score: 0,
        };
    }

    pub fn play_round(self: &mut Game, p1_choice: Choice, p2_choice: Choice) {
        self.p1_score += p1_choice.score();
        self.p2_score += p2_choice.score();

        match (p1_choice, p2_choice) {
            (p1_choice, p2_choice) if p1_choice.beats() == p2_choice => {
                // player 1 wins
                self.p1_score += WIN_POINTS;
                self.p2_score += LOSE_POINTS;
            }
            (p1_choice, p2_choice) if p2_choice.beats() == p1_choice => {
                // player 2 wins
                self.p1_score += LOSE_POINTS;
                self.p2_score += WIN_POINTS;
            }
            (p1_choice, p2_choice) if p1_choice == p2_choice => {
                // tie
                self.p1_score += TIE_POINTS;
                self.p2_score += TIE_POINTS;
            }
            (p1_choice, p2_choice) => panic!("Could not determine round outcome: {p1_choice:?}, {p2_choice:?}"),
        }
    }
}
