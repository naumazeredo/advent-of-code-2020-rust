pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let mut iter = buffer.trim().split_whitespace();

    let mut input = Vec::new();
    while let Some(s) = iter.next() {
        input.push(s.parse::<i32>().unwrap());
    }

    // O(N^2) solution
    {
        let numbers = input.clone();
        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    println!("{}", numbers[i] * numbers[j]);
                    break;
                }
            }
        }
    }

    // O(NlogN) solution
    {
        let mut numbers = input.clone();
        numbers.sort();
        for i in 0..numbers.len() {
            let rem = 2020 - numbers[i];
            match numbers.binary_search(&rem) {
                Ok(j) => {
                    println!("{}", numbers[i] * numbers[j]);
                    break;
                },
                _ => (),
            }
        }
    }

    // O(N) solution, if already sorted
    {
        let mut numbers = input.clone();
        numbers.sort();
        let mut l = 0;
        let mut r = numbers.len() - 1;

        loop {
            let sum = numbers[l] + numbers[r];
            if sum == 2020 {
                println!("{}", numbers[l] * numbers[r]);
                break;
            } else if sum < 2020 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    // O(N) expected
    {
        use std::collections::HashSet;
        let mut values = HashSet::new();

        let numbers = input.clone();

        for v in numbers.iter() {
            values.insert(v);
        }

        for x in numbers.iter() {
            if values.contains(&(2020 - x)) {
                println!("{}", x * (2020 - x));
                break;
            }
        }
    }
}

fn part2(buffer: String) {
    let mut iter = buffer.trim().split_whitespace();

    let mut input = Vec::new();
    while let Some(s) = iter.next() {
        input.push(s.parse::<i32>().unwrap());
    }


    // O(N^3)
    {
        let numbers = input.clone();
        let n = numbers.len();

        'outer1: for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        println!("{}", numbers[i] * numbers[j] * numbers[k]);
                        break 'outer1;
                    }
                }
            }
        }
    }

    // O(N^2logN)
    {
        let mut numbers = input.clone();
        numbers.sort();
        let n = numbers.len();

        'outer2: for i in 0..n {
            for j in i+1..n {
                let rem = 2020 - numbers[i] - numbers[j];
                match numbers.binary_search(&rem) {
                    Ok(k) => {
                        println!("{}", numbers[i] * numbers[j] * numbers[k]);
                        break 'outer2;
                    },
                    _ => (),
                }
            }
        }
    }

    // O(N^2 + NlogN)
    {
        let mut numbers = input.clone();
        numbers.sort();
        let n = numbers.len();

        'outer3: for i in 0..n {
            let mut l = i+1;
            let mut r = n-1;
            while l < r {
                let sum = numbers[i] + numbers[l] + numbers[r];
                if sum == 2020 {
                    println!("{}", numbers[i] * numbers[l] * numbers[r]);
                    break 'outer3;
                } else if sum < 2020 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
    }
}
