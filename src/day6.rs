pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let sum: u32 = buffer
        .split("\n\n")
        .map(|group| {
            let mut bitset = 0u32;

            group.lines()
                .filter(|s| s.len() > 0)
                .for_each(|s| {
                    assert!(s.is_ascii());
                    s.as_bytes().iter().for_each(|b| {
                        let pos = (*b).checked_sub(b'a').unwrap();
                        bitset |= 1 << pos;
                    });
                });

            bitset.count_ones()
        })
    .sum();

    println!("{}", sum);
}

fn part2(buffer: String) {
    let sum: u32 = buffer
        .split("\n\n")
        .map(|group| {
            let mut bitset_all = (1u32 << 26) - 1;

            group.lines()
                .filter(|s| s.len() > 0)
                .map(|s| {
                    assert!(s.is_ascii());
                    let mut bitset = 0u32;
                    s.as_bytes().iter().for_each(|b| {
                        let pos = (*b).checked_sub(b'a').unwrap();
                        bitset |= 1 << pos;
                    });
                    bitset
                })
            .for_each(|b| bitset_all &= b);

            bitset_all.count_ones()
        })
    .sum();

    println!("{}", sum);
}
