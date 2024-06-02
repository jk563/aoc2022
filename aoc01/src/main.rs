fn main() {
    let mut x: Vec<usize> = include_str!("real.data")
        .split("\n\n")
        .map(|packs| packs.lines().map(|raw| raw.parse::<usize>().unwrap()).sum())
        .collect();

    x.sort();
    x.reverse();

    println!("Highest: {}", x.iter().max().unwrap());
    println!("Sum of 3: {}", x.iter().take(3).sum::<usize>());
}
