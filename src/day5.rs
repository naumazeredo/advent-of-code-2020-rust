pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let max = buffer
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| {
            let number_str: String = s.chars().map(|c|
                match c {
                    'F' | 'L' => '0',
                    _ => '1',
                }
            ).collect();

            u32::from_str_radix(number_str.as_str(), 2).unwrap()
        })
        .max()
        .unwrap();

    println!("{}", max);
}

fn part2(buffer: String) {
    let mut ids = [false; 1024];

    buffer.lines()
           .filter(|s| s.len() > 0)
           .map(|s| {
               let number_str: String = s.chars().map(|c|
                   match c {
                       'F' | 'L' => '0',
                       _ => '1',
                   }
               ).collect();

               usize::from_str_radix(number_str.as_str(), 2).unwrap()
           })
           .for_each(|x| ids[x] = true);

    let mut id = 0;
    while !ids[id] { id += 1; }
    if id == 0 { id += 1; }
    while id < 1023 {
        if ids[id-1] && !ids[id] && ids[id+1] {
            break;
        }
        id += 1;
    }

    println!("{}", id);
}
