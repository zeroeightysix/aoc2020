use std::collections::HashMap;
use std::num::ParseIntError;

fn main() {
    const INPUT: &str = include_str!("input");
    let passport: Vec<HashMap<&str, &str>> = INPUT.split("\n\n").map(|passport| {
        passport.split_whitespace().map(|entry| {
            let mut split = entry.split(":");
            (split.next().unwrap(), split.next().unwrap())
        }).collect()
    }).collect();

    println!("Answer puzzle 1: {}", passport.iter().filter(is_valid).count());
    println!("Answer puzzle 2: {}", passport.iter().filter(is_valid).filter(|p| {
        match rules_satisfied(p) {
            Ok(x) => x,
            Err(_) => false
        }
    }).count());
}

fn is_valid(passport: &&HashMap<&str, &str>) -> bool {
    return ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|field| {
        passport.contains_key(*field)
    });
}

fn rules_satisfied(passport: &&HashMap<&str, &str>) -> Result<bool, ParseIntError> {
    let byr: usize = passport["byr"].parse()?;
    let iyr: usize = passport["iyr"].parse()?;
    let eyr: usize = passport["eyr"].parse()?;
    let height_good = {
        let height = passport["hgt"];
        if height.ends_with("cm") {
            let height: usize = height.trim_end_matches("cm").parse()?;
            (150..=193).contains(&height)
        } else if height.ends_with("in") {
            let height: usize = height.trim_end_matches("in").parse()?;
            (59..=76).contains(&height)
        } else {
            false
        }
    };
    let hcl = passport["hcl"];
    if !hcl.starts_with("#") || hcl.len() != 7 {
        return Ok(false)
    }
    let _hcl = usize::from_str_radix(&hcl[1..], 16)?;
    let ecl = passport["ecl"];
    let pid = passport["pid"];
    if pid.len() != 9 {
        return Ok(false)
    }
    let _pid: usize = pid.parse()?;

    Ok(
        (1920..=2002).contains(&byr)
            && (2010..=2020).contains(&iyr)
            && (2020..=2030).contains(&eyr)
            && height_good
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
    )
}
