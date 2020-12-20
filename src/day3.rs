pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

pub fn part1(buffer: String) {
    // No Iterator.first()?
    let cnt : u32 = calculate_slopes(buffer, vec![(3, 1)]).iter().product();
    println!("{}", cnt);
}

pub fn part2(buffer: String) {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let cnt : u32 = calculate_slopes(buffer, slopes).iter().product();
    println!("{}", cnt);
}

fn calculate_slopes(buffer: String, slopes: Vec<(usize, usize)>) -> Vec<u32> {
    let lines = buffer.split("\n").filter(|line| line.len() > 0);

    slopes
        .iter()
        .map(|(dx, dy)| {
            lines.clone()
                .step_by(*dy)
                .enumerate()
                .fold(0u32, |acc, (i, line)| {
                    acc + line.chars().cycle().step_by(*dx).nth(i).unwrap().eq(&'#') as u32
                })
        })
        .collect()
}
