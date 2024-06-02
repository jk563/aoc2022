use std::str::FromStr;
use std::cmp::Ordering;

fn main() {
    println!("Score 1: {}", score_rounds_1(include_str!("input.data")));
    println!("Score 2: {}", score_rounds_2(include_str!("input.data")));
}

fn score_round_2(round: &str) -> usize {
    let converted: Vec<Rps> = round.split(' ').map(|choice| choice.parse().unwrap()).collect();
    match converted[1] {
        Rps::Rock => match converted[0] {
            Rps::Rock => Rps::Scissors.value(),
            Rps::Paper => Rps::Rock.value(),
            Rps::Scissors => Rps::Paper.value(),
        },
        Rps::Paper => converted[0].value() + 3,
        Rps::Scissors => (match converted[0] {
            Rps::Rock => Rps::Paper.value(),
            Rps::Paper => Rps::Scissors.value(),
            Rps::Scissors => Rps::Rock.value(),
        }) + 6,
    }
}

fn score_rounds_2(rounds: &str) -> usize {
    rounds.lines().map(|round| score_round_2(round)).sum()
}

fn score_round_1(round: &str) -> usize {
    let converted: Vec<Rps> = round.split(' ').map(|choice| choice.parse().unwrap()).collect();
    if converted[1] > converted[0] {
        return converted[1].value() + 6
    } else if converted[1] == converted[0] {
        return converted[1].value() + 3
    } else {
        return converted[1].value()
    }
}

fn score_rounds_1(rounds: &str) -> usize {
    rounds.lines().map(|round| score_round_1(round)).sum()
}

#[derive(Debug, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Rps {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        match input {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, rhs: &Rps) -> Option<std::cmp::Ordering> {
        match *self {
            Rps::Rock => match rhs {
                    Rps::Rock => Some(Ordering::Equal),
                    Rps::Paper => Some(Ordering::Less),
                    Rps::Scissors => Some(Ordering::Greater),
                },
            Rps::Paper => match rhs {
                    Rps::Rock => Some(Ordering::Greater),
                    Rps::Paper => Some(Ordering::Equal),
                    Rps::Scissors => Some(Ordering::Less),
                },
            Rps::Scissors => match rhs {
                    Rps::Rock => Some(Ordering::Less),
                    Rps::Paper => Some(Ordering::Greater),
                    Rps::Scissors => Some(Ordering::Equal),
                },
        }
    }
}

impl Rps {
    fn value(&self) -> usize {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps_parse() {
        let actual_rock: Rps = "A".parse().unwrap();
        assert_eq!(Rps::Rock, actual_rock);
        let actual_rock: Rps = "X".parse().unwrap();
        assert_eq!(Rps::Rock, actual_rock);
        let actual_paper: Rps = "B".parse().unwrap();
        assert_eq!(Rps::Paper, actual_paper);
        let actual_paper: Rps = "Y".parse().unwrap();
        assert_eq!(Rps::Paper, actual_paper);
        let actual_scissors: Rps = "C".parse().unwrap();
        assert_eq!(Rps::Scissors, actual_scissors);
        let actual_scissors: Rps = "Z".parse().unwrap();
        assert_eq!(Rps::Scissors, actual_scissors);
    }

    #[test]
    fn test_rps_comparison() {
        assert!(Rps::Rock == Rps::Rock);
        assert!(Rps::Rock < Rps::Paper);
        assert!(Rps::Rock > Rps::Scissors);
        assert!(Rps::Paper > Rps::Rock);
        assert!(Rps::Paper == Rps::Paper);
        assert!(Rps::Paper < Rps::Scissors);
        assert!(Rps::Scissors < Rps::Rock);
        assert!(Rps::Scissors > Rps::Paper);
        assert!(Rps::Scissors == Rps::Scissors);
    }

    #[test]
    fn test_rps_value() {
        assert_eq!(Rps::Rock.value(), 1);
        assert_eq!(Rps::Paper.value(), 2);
        assert_eq!(Rps::Scissors.value(), 3);
    }

    #[test]
    fn test_score_round_1() {
        assert_eq!(score_round_1("A Y"), 8);
        assert_eq!(score_round_1("B X"), 1);
        assert_eq!(score_round_1("C Z"), 6);
    }

    #[test]
    fn test_test_data_1() {
        assert_eq!(score_rounds_1(include_str!("test.data")), 15);
    }
}
