fn main() {
    println!("Part 1: {}", part_1(include_str!("input.data")));
    println!("Part 2: {}", part_2(include_str!("input.data")));
}

fn letter_value(letter: char) -> usize {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _   => 0,
    }
}

fn part_1(data: &str) -> usize {
    data.lines().map(|backpack|  {
        let compartment_size = backpack.len() / 2;
        let compartment1 = &backpack[..compartment_size];
        let compartment2 = &backpack[compartment_size..];
        compartment1.chars().filter(|letter| compartment2.contains(*letter)).take(1).map(|matched_letter| letter_value(matched_letter)).sum::<usize>()
    }).sum::<usize>()
}

fn part_2(data: &str) -> usize {
    let groups: Vec<Vec<&str>> = data.lines().collect::<Vec<&str>>().chunks(3).map(|x| x.to_vec()).collect();
    groups.iter().map(|group| {
        group[0].chars().filter(|letter| group[1].contains(*letter) && group[2].contains(*letter)).take(1).map(|matched_letter| letter_value(matched_letter)).sum::<usize>()
    }).sum::<usize>()
}
