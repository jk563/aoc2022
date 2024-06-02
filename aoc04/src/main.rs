fn main() {
    let input = include_str!("input.data");
    println!("Part 1 Overlaps: {}", overlaps(input, &total));
    println!("Part 2 Overlaps: {}", overlaps(input, &partial));
}

fn overlaps(data: &str, strategy: &dyn Fn(Vec<usize>, Vec<usize>) -> bool) -> usize {
    data.lines().filter(|pair|  {
        let assignments: Vec<&str> = pair.split(',').collect();
        let elf1_assignment: Vec<usize> = assignments[0].split('-').map(|x| x.parse::<usize>().unwrap()).collect();
        let elf2_assignment: Vec<usize> = assignments[1].split('-').map(|x| x.parse::<usize>().unwrap()).collect();
        strategy(elf1_assignment, elf2_assignment)
    }).count()
}

fn total(elf1_assignment: Vec<usize>, elf2_assignment: Vec<usize>) -> bool {
    ((elf1_assignment[0] <= elf2_assignment[0]) && (elf1_assignment[1] >= elf2_assignment[1]))
        || ((elf1_assignment[0] >= elf2_assignment[0]) && (elf1_assignment[1] <= elf2_assignment[1]))
}

fn partial(elf1_assignment: Vec<usize>, elf2_assignment: Vec<usize>) -> bool {
    (elf1_assignment[0] <= elf2_assignment[0] && elf1_assignment[1] >= elf2_assignment[0])
        || (elf2_assignment[0] <= elf1_assignment[0] && elf2_assignment[1] >= elf1_assignment[0])
        || (elf1_assignment[0] <= elf2_assignment[1] && elf1_assignment[1] >= elf2_assignment[1])
        || (elf2_assignment[0] <= elf1_assignment[1] && elf2_assignment[1] >= elf1_assignment[1])
}

