use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::colors;

#[allow(dead_code)]
pub struct Rule {
    name: String,
    pattern: Regex,
    background: Color,
    foreground: Color,
    first_only: bool
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RawRule {
    name: String,
    pattern: String,
    ignore_case: bool,
    background: Color,
    foreground: Color,
    first_only: bool
}

#[derive(Deserialize)]
#[derive(Clone)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

pub fn get_standard_rules(set_name: &str) -> Result<Vec<Rule>, String> {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("rules")
        .join("sets")
        .join(format!("{}.json", set_name.to_lowercase()));
    
    return Ok(parse_rules(&path));
}

pub fn parse_rules(filename: &PathBuf) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();

    let contents = fs::read_to_string(filename)
        .expect(&format!("Should have been able to read the colorization rules file '{}'.",
                &filename.to_string_lossy()));

    let values: Vec<RawRule> = serde_json::from_str(&contents)
        .expect(&format!("Should have been able to parse the JSON array in the colorization rules file '{}'.",
        &filename.to_string_lossy()));

    for value in values.iter() {
        rules.push(Rule {
            name: value.name.clone(),
            pattern: RegexBuilder::new(&value.pattern)
                .unicode(false)
                .case_insensitive(value.ignore_case)
                .build()
                .unwrap(),
            foreground: value.foreground.clone(),
            background: value.background.clone(),
            first_only: value.first_only
        });
    }

    return rules;
}

pub fn apply_rules(rules: &Vec<Rule>, line: String) -> String {
    let mut result: String = line.to_owned();

    // First Only
    for rule in rules.iter().filter(|x| x.first_only) {
        let mut temp = "".to_owned();
        let Some(caps) = rule.pattern.captures(&result) else {
            continue;
        };     

        let start: usize;
        let end: usize;

        if caps.name("c") != None {
            let t = caps.name("c").unwrap();
            start = t.start();
            end = t.end();
        }
        else {
            let t = caps.get(0).unwrap();
            start = t.start();
            end = t.end();
        }

        temp.push_str(&result[..start]);
        temp.push_str(&get_color(rule.background.clone(), false));
        temp.push_str(&get_color(rule.foreground.clone(), true));
        temp.push_str(&result[start..end]);
        temp.push_str(&get_color(Color::Default, false));
        temp.push_str(&get_color(Color::Default, true));
        temp.push_str(&result[end..]);

        result = temp;
    }  

    // All
    for rule in rules.iter().filter(|x| !x.first_only) {
        let zz = &result.clone();
        let mut points: Vec<(usize, usize)> = rule.pattern.captures_iter(zz).map(|x| {
            if x.name("c") != None {
                let y = x.name("c").unwrap();
                (y.start(), y.end())
            }
            else {
                let y = x.get(0).unwrap();
                (y.start(), y.end())
            }
        }).collect();

        points.reverse();

        for (start, end) in points {
            let mut temp = "".to_owned();

            temp.push_str(&result[..start]);
            temp.push_str(&get_color(rule.background.clone(), false));
            temp.push_str(&get_color(rule.foreground.clone(), true));
            temp.push_str(&result[start..end]);
            temp.push_str(&get_color(Color::Default, false));
            temp.push_str(&get_color(Color::Default, true));
            temp.push_str(&result[end..]);

            result = temp;
        }
    }

    return result;
}

fn get_color(color: Color, foreground: bool) -> String {
    if foreground {
        match color {
            Color::Default => return colors::DEFAULT_FOREGROUND.to_string(),
            Color::Black => return colors::BLACK_FOREGROUND.to_string(),
            Color::Red => return colors::RED_FOREGROUND.to_string(),
            Color::Green => return colors::GREEN_FOREGROUND.to_string(),
            Color::Yellow => return colors::YELLOW_FOREGROUND.to_string(),
            Color::Blue => return colors::BLUE_FOREGROUND.to_string(),
            Color::Magenta => return colors::MAGENTA_FOREGROUND.to_string(),
            Color::Cyan => return colors::CYAN_FOREGROUND.to_string(),
            Color::White => return colors::WHITE_FOREGROUND.to_string(),
            Color::BrightBlack => return colors::BRIGHT_BLACK_FOREGROUND.to_string(),
            Color::BrightRed => return colors::BRIGHT_RED_FOREGROUND.to_string(),
            Color::BrightGreen => return colors::BRIGHT_GREEN_FOREGROUND.to_string(),
            Color::BrightYellow => return colors::BRIGHT_YELLOW_FOREGROUND.to_string(),
            Color::BrightBlue => return colors::BRIGHT_BLUE_FOREGROUND.to_string(),
            Color::BrightMagenta => return colors::BRIGHT_MAGENTA_FOREGROUND.to_string(),
            Color::BrightCyan => return colors::BRIGHT_CYAN_FOREGROUND.to_string(),
            Color::BrightWhite => return colors::BRIGHT_WHITE_FOREGROUND.to_string(),
        }
    }
    else {
        match color {
            Color::Default => return colors::DEFAULT_BACKGROUND.to_string(),
            Color::Black => return colors::BLACK_BACKGROUND.to_string(),
            Color::Red => return colors::RED_BACKGROUND.to_string(),
            Color::Green => return colors::GREEN_BACKGROUND.to_string(),
            Color::Yellow => return colors::YELLOW_BACKGROUND.to_string(),
            Color::Blue => return colors::BLUE_BACKGROUND.to_string(),
            Color::Magenta => return colors::MAGENTA_BACKGROUND.to_string(),
            Color::Cyan => return colors::CYAN_BACKGROUND.to_string(),
            Color::White => return colors::WHITE_BACKGROUND.to_string(),
            Color::BrightBlack => return colors::BRIGHT_BLACK_BACKGROUND.to_string(),
            Color::BrightRed => return colors::BRIGHT_RED_BACKGROUND.to_string(),
            Color::BrightGreen => return colors::BRIGHT_GREEN_BACKGROUND.to_string(),
            Color::BrightYellow => return colors::BRIGHT_YELLOW_BACKGROUND.to_string(),
            Color::BrightBlue => return colors::BRIGHT_BLUE_BACKGROUND.to_string(),
            Color::BrightMagenta => return colors::BRIGHT_MAGENTA_BACKGROUND.to_string(),
            Color::BrightCyan => return colors::BRIGHT_CYAN_BACKGROUND.to_string(),
            Color::BrightWhite => return colors::BRIGHT_WHITE_BACKGROUND.to_string(),
        }
    }
}