use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use rusttype::Font;

use font_loader::system_fonts;
use std::fs::File;
use std::fs::read_to_string;
use crate::log_time;
use crate::cli::CLIOptions;
use crate::wordcloud::WordCloud;
use std::path::PathBuf;
use crate::element::PositionedElement;
use serde::Serialize;

pub fn read_input_file(settings: &CLIOptions, blacklist: &HashSet<String>) -> Vec<(String, u16)> {
    let _log = log_time("reading input file");

    let regex = Regex::new(r"[^a-zA-Z\u0080-\uFFFF]").unwrap();
    let file = File::open(&settings.input_file).unwrap();

    // map words to their count: String -> u16
    let word_counts: HashMap<String, u16> = {
        let mut map = HashMap::new();

        BufReader::new(file)
            .lines()
            .flat_map(|opt_line| opt_line)
            // remote special characters
            .map(|line| (&regex.replace_all(&line, " ")).to_string())
            // split line into words
            .flat_map(|line|
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            )
            // apply min_word_length filter
            .filter(|word| word.chars().count() >= settings.min_word_length)
            // filter out words contained in blacklist
            .filter(|word| !blacklist.contains(&word.to_lowercase()))
            // apply to_lower_case
            .map(|word| if settings.to_lower_case { word.to_lowercase() } else { word.to_string() })
            // apply capitalize_words
            .map(|word| if settings.capitalize_words {
                word.char_indices().map(|(i, c)| match i {
                    0 => c.to_uppercase().nth(0).unwrap_or(c),
                    _ => c
                }).collect()
            } else {
                word
            })
            .for_each(|word| *map.entry(word).or_insert(0u16) += 1);

        map
    };

    let mut words: Vec<(String, u16)> = word_counts.into_iter().collect();
    words.sort_by(|w1, w2| w2.1.cmp(&w1.1));
    words.into_iter().take(settings.word_limit).collect()
}

pub fn read_blacklist_files(settings: &CLIOptions) -> HashSet<String> {
    let _log = log_time("reading blacklist file");

    let mut blacklist = HashSet::new();

    for blacklist_file in settings.blacklist_files.iter() {
        let content = read_to_string(blacklist_file).unwrap().replace("\n", " ");
        content
            .split(" ")
            .map(|word| word.to_lowercase())
            .for_each(|word| { blacklist.insert(word); });
    }

    blacklist
}

pub fn load_font(settings: &CLIOptions) -> Font {
    let _log = log_time("loading font");

    let font_data = {
        let property = system_fonts::FontPropertyBuilder::new()
            .family(&settings.font_name)
            .build();
        system_fonts::get(&property).expect("unable to find font").0
    };

    Font::from_bytes(font_data).expect("unable to parse font")
}

pub fn write_data_file(cloud: &WordCloud, path: &PathBuf) {
    let _log = log_time("generating dump");

    #[derive(Serialize)]
    struct SerializableVector { x: u32, y: u32 }

    #[derive(Serialize)]
    struct SerializableElement {
        text: String,
        position: SerializableVector,
        width: SerializableVector,
    }

    fn convert(element: &PositionedElement) -> SerializableElement {
        SerializableElement {
            text: element.element.text.to_owned(),
            position: SerializableVector {
                x: element.position.x as u32,
                y: element.position.y as u32,
            },
            width: SerializableVector {
                x: element.bounding_box.size.x,
                y: element.bounding_box.size.y,
            },
        }
    }

    let converted: Vec<_> = cloud.positioned_elements.iter().map(convert).collect();

    let json = serde_json::to_string(&converted).unwrap();

    std::fs::write(path, json).unwrap();
}