use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), std::io::Error> {
    println!("1.1:");
    let path = "day01";
    let buffered = BufReader::new(File::open(path)?);
    let lines: Vec<i64> = buffered
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    for i in 1..lines.len() {
        for j in i..lines.len() {
            if lines[i] + lines[j] == 2020 {
                println!("{}", lines[i] * lines[j]);
            }
        }
    }

    println!("\n1.2:");
    for i in 1..lines.len() {
        for j in i..lines.len() {
            for k in (i + j)..lines.len() {
                if lines[i] + lines[j] + lines[k] == 2020 {
                    println!("{}", lines[i] * lines[j] * lines[k]);
                }
            }
        }
    }

    println!("\n2.1:");

    let path = "day02";
    let buffered = BufReader::new(File::open(path)?);
    let mut valid = 0;
    for line in buffered.lines() {
        let line = line.unwrap();
        let mut words = line.split(" ");
        let mut minmax = words.next().unwrap().split("-");
        let min = minmax.next().unwrap().parse::<usize>().unwrap();
        let max = minmax.next().unwrap().parse::<usize>().unwrap();
        let letter = words.next().unwrap().chars().nth(0).unwrap();
        let password = words.next().unwrap();
        // println!("{}, {}, {}, {}", min, max, letter, password);

        let matches = password.matches(letter).count();
        if matches <= max && matches >= min {
            valid += 1;
        }
    }
    println!("{}", valid);

    println!("\n2.2:");

    let buffered = BufReader::new(File::open(path)?);
    let mut valid = 0;
    for line in buffered.lines() {
        let line = line.unwrap();
        let mut words = line.split(" ");
        let mut minmax = words.next().unwrap().split("-");
        let pos_a = minmax.next().unwrap().parse::<usize>().unwrap();
        let pos_b = minmax.next().unwrap().parse::<usize>().unwrap();
        let letter = words.next().unwrap().chars().nth(0).unwrap();
        let password = words.next().unwrap();

        let match_a = password.chars().nth(pos_a - 1).unwrap() == letter;
        let match_b = password.chars().nth(pos_b - 1).unwrap() == letter;

        if (match_a && !match_b) || (match_b && !match_a) {
            valid += 1;
        }
    }
    println!("{}", valid);

    println!("\n3.1:");

    let path = "day03";
    let mut pos_x = 0;
    let mut trees = 0;
    let buffered = BufReader::new(File::open(path)?);
    for line in buffered.lines() {
        let line = line.unwrap();
        let len = line.chars().count();
        let field = line.chars().nth(pos_x % len).unwrap();
        if field == '#' {
            trees += 1;
        }
        pos_x += 3;
    }
    println!("{}", trees);

    println!("\n3.2:");

    let configs: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees: [usize; 5] = [0; 5];
    for (tree, (step_x, step_y)) in configs.iter().enumerate() {
        let mut pos_x = 0;
        let mut pos_y = 0;
        let buffered = BufReader::new(File::open(path)?);
        for (n, line) in buffered.lines().enumerate() {
            if n != pos_y {
                continue;
            }
            let line = line.unwrap();
            let len = line.chars().count();
            let field = line.chars().nth(pos_x % len).unwrap();
            if field == '#' {
                trees[tree] += 1;
            }
            pos_x += step_x;
            pos_y += step_y;
        }
    }
    println!("{}", trees.iter().fold(1, |a, b| a * b));

    println!("\n4.1:");

    use aoc2020::{fill_passport, Passport};

    let path = "day04";
    let buffered = BufReader::new(File::open(path)?);

    let mut passports: Vec<Passport> = Vec::new();

    let mut passport_lines = Vec::new();
    for line in buffered.lines() {
        let line = line.unwrap();

        match &line[..] {
            "" => {
                let passport: String = passport_lines.join(" ");
                // println!("{}", passport);
                passports.push(fill_passport(&passport));
                passport_lines.clear();
            }
            _ => passport_lines.push(line),
        };
    }
    let passport: String = passport_lines.join(" ");
    // println!("{}", passport);
    passports.push(fill_passport(&passport));

    let valid_cnt = passports
        .iter()
        .map(|p| p.check_passport())
        .fold(0, |cnt, val| if val { cnt + 1 } else { cnt });
    println!("{}", valid_cnt);

    println!("\n4.2:");

    let valid_cnt = passports
        .iter()
        .filter(|p| p.check_passport())
        .map(|p| p.validate_passport())
        .fold(0, |cnt, val| if val { cnt + 1 } else { cnt });
    println!("{}", valid_cnt);

    println!("\n5.1:");

    let path = "day05";
    let buffered = BufReader::new(File::open(path)?);
    let mut max = 0;
    for line in buffered.lines() {
        let line = line.unwrap();
        let row = &line[..7];
        let col = &line[7..10];
        // println!("row {} col {}", row, col);
        let row: String = row
            .chars()
            .map(|c| if c == 'F' { '0' } else { '1' })
            .collect();
        let col: String = col
            .chars()
            .map(|c| if c == 'L' { '0' } else { '1' })
            .collect();
        // println!("row {} col {}", row, col);
        let row = usize::from_str_radix(&row, 2).unwrap();
        let col = usize::from_str_radix(&col, 2).unwrap();
        // println!("row {} col {}", row, col);
        let id = 8 * row + col;
        if id > max {
            // println!("row {} col {}", row, col);
            max = id;
        }
    }
    println!("{}", max);

    println!("\n5.2:");

    let seats = vec![0; (2 as usize).pow(10)];
    let mut ids = Vec::new();
    let mut seats = seats
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let buffered = BufReader::new(File::open(path)?);
    for line in buffered.lines() {
        let line = line.unwrap();
        let row = &line[..7];
        let col = &line[7..10];
        // println!("row {} col {}", row, col);
        let row: String = row
            .chars()
            .map(|c| if c == 'F' { '0' } else { '1' })
            .collect();
        let col: String = col
            .chars()
            .map(|c| if c == 'L' { '0' } else { '1' })
            .collect();
        // println!("row {} col {}", row, col);
        let row = usize::from_str_radix(&row, 2).unwrap();
        let col = usize::from_str_radix(&col, 2).unwrap();
        // println!("row {} col {}", row, col);
        let id = 8 * row + col;
        ids.push(id);
        seats.retain(|x| *x != id);
    }

    for seat in seats {
        if (seat == 0) {
            continue;
        }
        if ids.contains(&(seat - 1)) && ids.contains(&(seat + 1)) {
            println!("{}", seat);
        }
    }

    println!("\n6.1:");

    Ok(())
}
