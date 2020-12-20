pub fn run(part: &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let mut ans = 0;

    let mut iter = buffer.trim().lines();
    while let Some(line) = iter.next() {
        let split : Vec<&str> = line.split(|c : char| c == ':' || c == '-' || c.is_whitespace()).collect();
        assert!(split.len() >= 4);

        let min = split[0].parse::<u32>().unwrap();
        let max = split[1].parse::<u32>().unwrap();
        assert!(min <= max);

        let letter = split[2].parse::<char>().unwrap();
        let string = split[4];

        let cnt = string
            .chars()
            .fold(0u32, |acc, x| acc + (x == letter) as u32);

        if cnt >= min && cnt <= max {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn part2(buffer: String) {
    let mut ans = 0;

    let mut iter = buffer.trim().lines();
    while let Some(line) = iter.next() {
        let split : Vec<&str> = line.split(|c : char| c == ':' || c == '-' || c.is_whitespace()).collect();
        assert!(split.len() >= 4);

        let min = split[0].parse::<usize>().unwrap();
        let max = split[1].parse::<usize>().unwrap();
        assert!(min <= max);

        let letter = split[2].parse::<char>().unwrap();
        let string : Vec<char> = split[4].chars().collect();
        assert!(string.len() >= max);

        if (string[min-1] == letter) ^ (string[max-1] == letter) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
