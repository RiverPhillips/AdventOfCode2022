use std::env::args;
use std::fs;
use std::str::FromStr;

fn main() {
    let args : Vec<String> = args().skip(1).collect();

    let file_contents = fs::read_to_string(&args[0]).expect("Could not read file");
    let calories = sum_elves_calories(&file_contents);
    println!("Max Calories: {}", calories.iter().max().expect("No max calories"));
    println!("Top 3: {:?}", top_three_sum(calories))
}

fn sum_elves_calories(input: &str) -> Vec<i32> {
    input.trim().split("\n\n").map(|x|{
        x.split("\n").map(|z| {
            let val: i32 = match FromStr::from_str(z) {
                Ok(x) => x,
                Err(err) => panic!("Failed to convert {}. Error: {}.", z, err),
            };
            return val
        }).sum()
    }).collect::<Vec<i32>>()
}

fn top_three_sum(input: Vec<i32>) -> i32 {
    let mut top_three = vec![0, 0, 0];
    for i in input {
        if i > top_three[0] {
            top_three[0] = i;
            top_three.sort();
        }
    }
   top_three.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::{top_three_sum, sum_elves_calories};

    #[test]
    fn it_sums_all_elves_calories() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;

        assert_eq!(
            sum_elves_calories(input),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn it_gets_the_top_3() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(top_three_sum(input), 45000);
    }
}
