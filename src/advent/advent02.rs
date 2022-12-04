use crate::advent::Advent;

pub struct Advent02;

pub enum RPS {
    Rock,
    Paper,
    Scissors
}

pub struct Turn ( RPS, RPS );

use RPS::{Rock,Paper,Scissors};

impl Advent<Turn> for Advent02 {
    fn transform(line: &str) -> Turn {
        let opponent = match line.chars().nth(0).unwrap() {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _  => panic!("argh!")
        };
        let player = match line.chars().nth(2).unwrap() {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _  => panic!("argh!")
        };
        Turn(opponent, player)
    }

    fn part_one(turns: &Vec<Turn>) {
      let scores =  turns.into_iter().map(|Turn (opponent, player)| {
        let score_choose = match player {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let score_turn = match (opponent, player) {
            (RPS::Rock, RPS::Rock) => 3,
            (RPS::Rock, RPS::Paper) => 6,
            (RPS::Rock, RPS::Scissors) => 0,
            (RPS::Paper, RPS::Rock) => 0,
            (RPS::Paper, RPS::Paper) => 3,
            (RPS::Paper, RPS::Scissors) => 6,
            (RPS::Scissors, RPS::Rock) => 6,
            (RPS::Scissors, RPS::Paper) => 0,
            (RPS::Scissors, RPS::Scissors) => 3,
        };
        score_choose + score_turn
    });
        println!("Your score is: {:?}" , scores.into_iter().reduce(|a,b| a+b).unwrap());
    }

    fn part_two(data: &Vec<Turn>) {
        
    }
}