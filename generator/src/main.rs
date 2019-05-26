mod element;
mod spiral;
mod io;
mod wordcloud;
#[macro_use]
mod graphics;
mod cli;

use element::Element;
use std::time::Instant;
use rusttype::Font;
use crate::cli::CLIOptions;

use structopt::StructOpt;
use rand::Rng;

fn main() {
    let _log = log_time("running");

    let settings: CLIOptions = CLIOptions::from_args();
    let blacklist = io::read_blacklist_files(&settings);
    let words = io::read_input_file(&settings, &blacklist);
    let font = io::load_font(&settings);
    let mut elements = create_elements(words, &font, &settings);

    let mut cloud = wordcloud::WordCloud::new(&settings);

    assert!(elements.len() > 0);

    // positioning elements
    {
        let _log = log_time("positioning elements");

        let element_count = elements.len();
        let chunks = (settings.word_limit as f32 / 10.0).ceil() as usize;
        let chunk_size = element_count / chunks;
        for (index, element) in elements.into_iter().enumerate() {
            if index % chunk_size == 0 {
                cloud.spiral.index = 0.0;
            }
            if index % settings.log_every_n_elements == 0 {
                println!("... {}/{}", index, element_count);
            }
            cloud.position(element);
        }
    }


    // render image
    let result = {
        let mut image = cloud.render(&settings);

        if settings.post_processing_rotation != 0.0 {
            image = graphics::rotate(&settings, image);
        }
        if let Some(background) = settings.background_color {
            graphics::set_background(&mut image, &background);
        }

        image
    };

    // saving data
    if let Some(data_output_file) = &settings.data_output_file {
        io::write_data_file(&cloud, &data_output_file);
    }


    // saving image
    {
        let _log = log_time("saving image");
        result.save(settings.output_file).unwrap();
    }
}


fn create_elements<'a>(words: Vec<(String, u16)>,
                       font: &'a Font<'a>,
                       settings: &CLIOptions) -> Vec<Element<'a>> {
    let max_n = words.iter().map(|w| w.1s).max().unwrap_or(0) as f32;
    let mut rng = rand::thread_rng();

    words.iter()
        .map(|(word, n)| {
            let mut element = Element::new(word.to_owned(),
                                           *n as f32 / max_n * settings.resolution as f32,
                                           font);

            if rng.gen_ratio(settings.rotate_percentage, 100) {
                element.rotate()
            }

            element
        })
        .collect()
}

pub fn log_time<'a>(function: &'static str) -> LogTime {
    println!("--> {}", function);
    LogTime { function, start: Instant::now() }
}

pub struct LogTime<'a> {
    function: &'a str,
    start: Instant,
}

impl<'a> Drop for LogTime<'a> {
    fn drop(&mut self) {
        println!("<-- {} ({:?})", self.function, self.start.elapsed());
    }
}
