use structopt::StructOpt;
use std::path::PathBuf;
use image::Rgb;
use std::fs::File;

#[derive(Debug, StructOpt)]
pub struct CLIOptions {
    #[structopt(long = "input-file", parse(try_from_str = "ensure_file_exists"))]
    pub input_file: PathBuf,
    #[structopt(long = "blacklist", parse(try_from_str = "ensure_file_exists"))]
    pub blacklist_files: Vec<PathBuf>,
    #[structopt(long = "to-lower-case")]
    pub to_lower_case: bool,
    #[structopt(long = "font", default_value = "Arial")]
    pub font_name: String,
    #[structopt(long = "word-limit", default_value = "50")]
    pub word_limit: usize,
    #[structopt(long = "min-word-length", default_value = "3")]
    pub min_word_length: usize,
    #[structopt(long = "capitalize-words")]
    pub capitalize_words: bool,
    #[structopt(long = "spiral-step", default_value = "0.05")]
    pub spiral_step: f32,
    #[structopt(long = "log-every-n-elements", default_value = "10")]
    pub log_every_n_elements: usize,
    #[structopt(long = "post-processing-rotation", default_value = "60.0")]
    pub post_processing_rotation: f32,
    #[structopt(long = "resolution", default_value = "300")]
    pub resolution: usize,
    #[structopt(long = "font-color", default_value = "#333333", parse(try_from_str = "parse_color"))]
    pub font_color: Rgb<u8>,
    #[structopt(long = "background-color", parse(try_from_str = "parse_color"))]
    pub background_color: Option<Rgb<u8>>,
    #[structopt(long = "output-file", default_value = "output.png")]
    pub output_file: PathBuf,
    #[structopt(long = "data-output-file")]
    pub data_output_file: Option<PathBuf>,
    #[structopt(long = "rotate-percentage", default_value = "20", parse(try_from_str = "ensure_percentage"))]
    pub rotate_percentage: u32,
}


fn ensure_percentage(s: &str) -> Result<u32, String> {
    let percentage = match s.parse::<u32>() {
        Ok(percentage) => percentage,
        Err(_) => return Err(format!("expected int between 0 and 100, got {}", s))
    };

    if  percentage > 100 {
        Err(format!("expected int between 0 and 100, got {}", s))
    } else {
        Ok(percentage)
    }
}

fn ensure_file_exists(s: &str) -> Result<PathBuf, String> {
    match File::open(s) {
        Ok(_) => Ok(PathBuf::from(s)),
        Err(err) => Err(format!("{} is not a valid file: {:?}", s, err.kind()))
    }
}

fn parse_color(s: &str) -> Result<Rgb<u8>, String> {
    if s.len() != 7 || !s.starts_with('#') {
        return Err("expected #XXXXXX, where X=0..F".to_owned());
    }

    let red_hex = &s.chars().skip(1).take(2).collect::<String>();
    let green_hex = &s.chars().skip(3).take(2).collect::<String>();
    let blue_hex = &s.chars().skip(5).take(2).collect::<String>();
    let red = u8::from_str_radix(red_hex, 16)
        .map_err(|_| "expected 2 hex digits".to_string())?;
    let green = u8::from_str_radix(green_hex, 16)
        .map_err(|_| "expected 2 hex digits".to_string())?;
    let blue = u8::from_str_radix(blue_hex, 16)
        .map_err(|_| "expected 2 hex digits".to_string())?;

    Ok(Rgb([red, green, blue]))
}