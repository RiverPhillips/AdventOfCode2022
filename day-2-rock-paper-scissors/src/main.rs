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

lazy_static!{
    static ref OPPONENT_HASH_MAP: HashMap<char, Moves> = HashMap::from([
        ('A', Moves::Rock),
        ('B', Moves::Paper),
        ('C',  Moves::Scissors),
    ]);

    static ref MY_HASH_MAP: HashMap<char, Moves> = HashMap::from([
        ('X', Moves::Rock),
        ('Y', Moves::Paper),
        ('Z',  Moves::Scissors),
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
    file_contents.split("\n").map(|row| {
        let moves = get_moves_from_row(row);
        let result = get_result(moves.1, moves.0);
        i32::from(get_score(&result, moves.1))
    }).sum::<i32>()
}

fn get_result(my_move: &Moves, opponent_move: &Moves) -> RockPaperScissorsResult {
    match (my_move, opponent_move) {
        (Moves::Rock, Moves::Rock) => RockPaperScissorsResult::Draw,
        (Moves::Rock, Moves::Paper) => RockPaperScissorsResult::Loss,
        (Moves::Rock, Moves::Scissors) => RockPaperScissorsResult::Win,
        (Moves::Paper, Moves::Rock) => RockPaperScissorsResult::Win,
        (Moves::Paper, Moves::Paper) => RockPaperScissorsResult::Draw,
        (Moves::Paper, Moves::Scissors) => RockPaperScissorsResult::Loss,
        (Moves::Scissors, Moves::Rock) => RockPaperScissorsResult::Loss,
        (Moves::Scissors, Moves::Paper) => RockPaperScissorsResult::Win,
        (Moves::Scissors, Moves::Scissors) => RockPaperScissorsResult::Draw,
    }
}

fn get_score(result: &RockPaperScissorsResult, my_move: &Moves) -> i8 {
    let result_score = RESULT_TO_SCORE.get(result).unwrap();
    let move_score = MOVE_TO_SCORE.get(my_move).unwrap();
    result_score + move_score
}

fn get_moves_from_row(row: &str) -> (&Moves, &Moves) {
    let mut chars = row.chars();

    let opponent_move: &Moves = OPPONENT_HASH_MAP.get(&chars.nth(0).unwrap()).unwrap();
    let my_move: &Moves = MY_HASH_MAP.get(&chars.nth(1).unwrap()).unwrap();

    return (opponent_move, my_move)
}


#[cfg(test)]
mod test {
    use crate::{calculate_total_score, get_moves_from_row, get_result, get_score, Moves, RockPaperScissorsResult};

    #[test]
    fn calculates_the_correct_result(){
        let test_cases = [(Moves::Rock, Moves::Rock, RockPaperScissorsResult::Draw),
                         (Moves::Rock, Moves::Paper, RockPaperScissorsResult::Loss),
                         (Moves::Rock, Moves::Scissors, RockPaperScissorsResult::Win),
                         (Moves::Paper, Moves::Rock, RockPaperScissorsResult::Win),
                         (Moves::Paper, Moves::Paper, RockPaperScissorsResult::Draw),
                         (Moves::Paper, Moves::Scissors, RockPaperScissorsResult::Loss),
                         (Moves::Scissors, Moves::Rock, RockPaperScissorsResult::Loss),
                         (Moves::Scissors, Moves::Paper, RockPaperScissorsResult::Win),
                         (Moves::Scissors, Moves::Scissors, RockPaperScissorsResult::Draw)];

        for test_case in test_cases {
            assert_eq!(get_result(&test_case.0,  &test_case.1), test_case.2);
        }
    }

    #[test]
    fn calculates_the_correct_score() {
        let test_cases = [
            (RockPaperScissorsResult::Loss, Moves::Rock, 1),
            (RockPaperScissorsResult::Draw, Moves::Rock, 4),
            (RockPaperScissorsResult::Win, Moves::Rock, 7),
            (RockPaperScissorsResult::Loss, Moves::Paper, 2),
            (RockPaperScissorsResult::Draw, Moves::Paper, 5),
            (RockPaperScissorsResult::Win, Moves::Paper, 8),
            (RockPaperScissorsResult::Loss, Moves::Scissors, 3),
            (RockPaperScissorsResult::Draw, Moves::Scissors, 6),
            (RockPaperScissorsResult::Win, Moves::Scissors, 9),
        ];

        for test_case in test_cases {
            assert_eq!(get_score(&test_case.0, &test_case.1), test_case.2);
        }
    }

    #[test]
    fn it_gets_moves_from_valid_rows(){
        let test_cases = [
            ("A X", Moves::Rock, Moves::Rock),
            ("A Y", Moves::Rock, Moves::Paper),
            ("A Z", Moves::Rock, Moves::Scissors),
            ("B X", Moves::Paper, Moves::Rock),
            ("B Y",Moves::Paper,Moves::Paper),
            ("B Z",Moves::Paper, Moves::Scissors),
            ("C X", Moves::Scissors, Moves::Rock),
            ("C Y",Moves::Scissors, Moves::Paper),
            ("C Z",Moves::Scissors, Moves::Scissors),
        ];

        for test_case in test_cases {
            assert_eq!(get_moves_from_row(test_case.0), (&test_case.1, &test_case.2));
        }
    }

    #[test]
    fn it_calculates_the_correct_total_score(){
        let input = r#"A X"#;

        assert_eq!(calculate_total_score(input), 15);
    }


}