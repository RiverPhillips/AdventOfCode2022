use std::collections::HashMap;
use std::{env, fs};

#[macro_use]
extern crate lazy_static;
extern crate core;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum RockPaperScissorsResult {
    Win,
    Loss,
    Draw
}

struct Row<'a> {
    opponent_move: &'a Moves,
    desired_outcome: &'a RockPaperScissorsResult,
}

lazy_static!{
    static ref OPPONENT_HASH_MAP: HashMap<char, Moves> = HashMap::from([
        ('A', Moves::Rock),
        ('B', Moves::Paper),
        ('C',  Moves::Scissors),
    ]);

    static ref DESIRED_RESULT_MAP: HashMap<char, RockPaperScissorsResult> = HashMap::from([
        ('X', RockPaperScissorsResult::Loss),
        ('Y', RockPaperScissorsResult::Draw),
        ('Z',  RockPaperScissorsResult::Win),
    ]);

    static ref RESULT_TO_SCORE: HashMap<RockPaperScissorsResult, i8> = HashMap::from([
        (RockPaperScissorsResult::Win, 6),
        (RockPaperScissorsResult::Draw, 3),
        (RockPaperScissorsResult::Loss, 0),
    ]);

    static ref MOVE_TO_SCORE: HashMap<Moves, i8> = HashMap::from([
        (Moves::Rock, 1),
        (Moves::Paper, 2),
        (Moves::Scissors, 3),
    ]);
}


fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    let file_contents = fs::read_to_string(&args[0]).expect("Could not read file");

    let score = calculate_total_score(file_contents.trim());

    println!("Score: {}", score);
}

fn calculate_total_score(file_contents: &str) -> i32 {
    file_contents.split("\n").map(|row_string| {
        let row = parse_row(row_string);
        let my_move = get_required_move(row.opponent_move, row.desired_outcome);
        i32::from(get_score(row.desired_outcome, my_move))
    }).sum::<i32>()
}

fn get_required_move(opponent_move: &Moves, desired_result: &RockPaperScissorsResult) -> Moves {
   match (opponent_move, desired_result) {
       (Moves::Rock, RockPaperScissorsResult::Win) => Moves::Paper,
       (Moves::Rock, RockPaperScissorsResult::Draw) => Moves::Rock,
       (Moves::Rock, RockPaperScissorsResult::Loss) => Moves::Scissors,
       (Moves::Paper, RockPaperScissorsResult::Win) => Moves::Scissors,
       (Moves::Paper, RockPaperScissorsResult::Draw) => Moves::Paper,
       (Moves::Paper, RockPaperScissorsResult::Loss) => Moves::Rock,
       (Moves::Scissors, RockPaperScissorsResult::Win) => Moves::Rock,
       (Moves::Scissors, RockPaperScissorsResult::Draw) => Moves::Scissors,
       (Moves::Scissors, RockPaperScissorsResult::Loss) => Moves::Paper,
   }
}

fn get_score(result: &RockPaperScissorsResult, my_move: Moves) -> i8 {
    let result_score = RESULT_TO_SCORE.get(result).unwrap();
    let move_score = MOVE_TO_SCORE.get(&my_move).unwrap();
    result_score + move_score
}

fn parse_row(row: &str) -> Row {
    let mut chars = row.chars();

    let opponent_move: &Moves = OPPONENT_HASH_MAP.get(&chars.nth(0).unwrap()).unwrap();
    let desired_outcome: &RockPaperScissorsResult= DESIRED_RESULT_MAP.get(&chars.nth(1).unwrap()).unwrap();

    return Row{
        opponent_move,
        desired_outcome,
    }
}


#[cfg(test)]
mod test {
    use crate::{calculate_total_score};


    #[test]
    fn it_calculates_the_correct_total_score(){
        let input = r#"A Y
B X
C Z"#;

        assert_eq!(calculate_total_score(input), 15);
    }


}