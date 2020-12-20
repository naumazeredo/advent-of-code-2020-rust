use std::collections::HashMap;

pub fn run(part : &str, buffer: String) {
    match part {
        "1" => part1(buffer),
        "2" => part2(buffer),
        _ => (),
    }
}

fn part1(buffer: String) {
    let fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    let cnt = buffer
        .split("\n\n")
        .filter(|s| s.len() > 0)
        .fold(0u32, |acc, line| {
            acc + fields.iter()
                        .fold(0u32, |acc, field| {
                            acc + (line.contains(field) as u32)
                        })
                        .eq(&7) as u32
        });

    println!("{}", cnt);
}

fn part2(buffer: String) {
    let cnt = buffer
        .split("\n\n")
        .filter(|s| s.len() > 0)
        .fold(0u32, |acc, line| {
            let mut fields = HashMap::new();
            line.split_whitespace()
                .filter(|s| s.len() > 0)
                .for_each(|s| {
                    let mut iter = s.split(":");
                    let name = iter.next().unwrap();
                    let value = iter.next().unwrap();
                    fields.insert(name, value);
                });

            acc + (is_valid(fields).is_ok() as u32)
        });

    println!("{}", cnt);
}

fn is_valid(fields : HashMap<&str, &str>) -> Result<(), ()> {
    let get_name = |name: &str| -> Result<&str, ()> {
        Ok(fields.get(name).ok_or_else(|| ())?)
    };

    let range_check = |name, range: (u32, u32)| {
        let value = get_name(name)?;
        if value.len() != 4 { return Err(()); }
        let value = match value.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err(()),
        };
        if value < range.0 || value > range.1 { return Err(()); }
        Ok(())
    };

    range_check("byr", (1920, 2002))?;
    range_check("iyr", (2010, 2020))?;
    range_check("eyr", (2020, 2030))?;

    let hgt_check = || {
        let value = get_name("hgt")?;
        if let Some(v) = value.strip_suffix("cm") {
            let value = match v.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

            if value < 150 || value > 193 { return Err(()); }
        } else if let Some(v) = value.strip_suffix("in") {
            let value = match v.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

            if value < 59 || value > 76 { return Err(()); }
        } else {
            return Err(());
        }

        Ok(())
    };

    hgt_check()?;

    let hcl_check = || {
        let value = get_name("hcl")?;
        let value = value.strip_prefix("#").ok_or_else(|| ())?;
        let is_hex_lowercase = value.chars().all(|c| {
            c.is_ascii_digit() ||
                (c.is_ascii_alphabetic() &&
                 c.is_ascii_hexdigit() &&
                 c.is_lowercase())
        });

        if !is_hex_lowercase { return Err(()); }
        Ok(())

        /*
        match std::i64::from_str_radix(value, 16) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
        */
    };

    hcl_check()?;

    let ecl_check = || {
        let value = get_name("ecl")?;
        match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
            _ => Err(()),
        }
    };

    ecl_check()?;

    let pid_check = || {
        let value = fields.get("pid").ok_or_else(|| ())?;
        if value.len() != 9 { return Err(()); }
        let is_number = value.chars().all(|c| c.is_ascii_digit());
        if !is_number { return Err(()); }
        Ok(())
    };

    pid_check()?;

    Ok(())
}
