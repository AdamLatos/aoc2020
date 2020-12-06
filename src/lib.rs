#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<String>, //(Birth Year)
    iyr: Option<String>, //(Issue Year)
    eyr: Option<String>, //(Expiration Year)
    hgt: Option<String>, //(Height)
    hcl: Option<String>, //(Hair Color)
    ecl: Option<String>, //(Eye Color)
    pid: Option<String>, //(Passport ID)
    cid: Option<String>, //(Country ID)
}

impl Passport {
    pub fn check_passport(&self) -> bool {
        match self {
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
            } => true,
            _ => false,
        }
    }

    pub fn validate_passport(&self) -> bool {
        // println!("{:?}", self);
        if !self.check_passport() {
            // println!("check passport false");
            return false;
        }
        // byr
        let byr = match self.byr.as_ref().unwrap().parse::<usize>() {
            Ok(n) => n,
            _ => {
                // println!("check byr false");
                return false;
            }
        };
        if byr < 1920 || byr > 2002 {
            // println!("byr range");
            return false;
        }
        // iyr
        let iyr = match self.iyr.as_ref().unwrap().parse::<usize>() {
            Ok(n) => n,
            _ => return false,
        };
        if iyr < 2010 || iyr > 2020 {
            // println!("iyr range");
            return false;
        }
        // eyr
        let eyr = match self.eyr.as_ref().unwrap().parse::<usize>() {
            Ok(n) => n,
            _ => return false,
        };
        if eyr < 2020 || eyr > 2030 {
            // println!("eyr range");
            return false;
        }
        // hgt
        let hgt = self.hgt.as_ref().unwrap();
        if hgt.ends_with("cm") {
            let height = hgt.strip_suffix("cm").unwrap().parse::<usize>().unwrap();
            if height < 150 || height > 193 {
                // println!("hgt range");
                return false;
            }
        } else if hgt.ends_with("in") {
            let height = hgt.strip_suffix("in").unwrap().parse::<usize>().unwrap();
            if height < 59 || height > 76 {
                // println!("hgt range");
                return false;
            }
        } else {
            // println!("hgt format");
            return false;
        }
        // hcl
        let hcl = self.hcl.as_ref().unwrap();
        if hcl.starts_with("#") && hcl.len() == 7 {
            let haircolor = usize::from_str_radix(hcl.strip_prefix("#").unwrap(), 16);
            match haircolor {
                Err(_) => {
                    // println!("hcl number");
                    return false;
                }
                _ => {}
            }
        } else {
            // println!("hcl format");
            return false;
        }
        // ecl
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl = &self.ecl.as_ref().unwrap()[..];
        if !colors.iter().any(|&c| c == ecl) {
            // println!("not in colors");
            return false;
        }
        // pid
        let pid = self.pid.as_ref().unwrap();
        if pid.len() != 9 {
            // println!("pid len: {}", pid.len());
            return false;
        }
        let pid = pid.parse::<usize>();
        match pid {
            Err(_) => {
                // println!("pid format");
                return false;
            }
            _ => {}
        }
        true
    }
}

pub fn fill_passport(data: &str) -> Passport {
    let mut passport = Passport::default();
    for term in data.split_whitespace() {
        let mut item = term.split(":");
        let field = item.next().unwrap();
        let value = item.next().unwrap();
        // println!("{}: {}", field, value);
        match field {
            "byr" => passport.byr = Some(value.to_string()),
            "iyr" => passport.iyr = Some(value.to_string()),
            "eyr" => passport.eyr = Some(value.to_string()),
            "hgt" => passport.hgt = Some(value.to_string()),
            "ecl" => passport.ecl = Some(value.to_string()),
            "hcl" => passport.hcl = Some(value.to_string()),
            "pid" => passport.pid = Some(value.to_string()),
            "cid" => passport.cid = Some(value.to_string()),
            _ => {}
        };
    }
    passport
}

#[cfg(test)]
mod tests {
    use crate::fill_passport;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn passport_correct() {
        let p = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";
        let p = fill_passport(p);
        assert!(p.check_passport());
    }

    #[test]
    fn passport_incorrect() {
        let p = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";
        let p = fill_passport(p);
        assert!(!p.check_passport());
    }

    #[test]
    fn passport_cheat() {
        let p = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";
        let p = fill_passport(p);
        assert!(p.check_passport());
    }
}
