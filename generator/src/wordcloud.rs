use crate::element::{Element, PositionedElement};
use crate::log_time;
use crate::spiral::Spiral;
use image::{RgbaImage, DynamicImage};
use rusttype::{point, vector};
use std::convert::TryFrom;
use crate::cli::CLIOptions;

pub struct WordCloud<'a> {
    pub positioned_elements: Vec<PositionedElement<'a>>,
    pub spiral: Spiral,
}

impl<'a> WordCloud<'a> {
    pub fn new(settings: &CLIOptions) -> Self {
        WordCloud {
            positioned_elements: Vec::with_capacity(settings.word_limit),
            spiral: Spiral::new(settings.spiral_step),
        }
    }

    pub fn position(&mut self, element: Element<'a>) -> &PositionedElement<'a> {
        let positioned = if self.positioned_elements.is_empty() {
            let size = element.bounding_box.size;
            let initial_loc = point(-(size.x as i32) / 2, -(size.y as i32) / 2);
            PositionedElement::new(initial_loc, element)
        } else {
            let mut positioned = PositionedElement::new(self.spiral.next_point(), element);
            while self.positioned_elements.iter().any(|e| positioned.collides_with(e)) {
                positioned = PositionedElement::new(self.spiral.next_point(), positioned.element);
            }

            positioned
        };

        self.positioned_elements.push(positioned);
        &self.positioned_elements.last().unwrap()
    }

    pub fn render(&mut self, settings: &CLIOptions) -> RgbaImage {
        let _log = log_time("rendering");

        let min_x = self.positioned_elements.iter()
            .map(|e| e.position.x).min().unwrap();
        let min_y = self.positioned_elements.iter()
            .map(|e| e.position.y).min().unwrap();
        let max_x = self.positioned_elements.iter()
            .map(|e| e.position.x + e.element.bounding_box.size.x as i32).max().unwrap();
        let max_y = self.positioned_elements.iter()
            .map(|e| e.position.y + e.element.bounding_box.size.y as i32).max().unwrap();

        let (width, height) = (u32::try_from(max_x - min_x).unwrap(),
                               u32::try_from(max_y - min_y).unwrap());
        let mut image: RgbaImage = DynamicImage::new_rgba8(width, height).to_rgba();

        let element_count = self.positioned_elements.len();

        for (index, mut element) in self.positioned_elements.iter_mut().enumerate() {
            if index % settings.log_every_n_elements == 0 {
                println!("... {}/{}", index, element_count);
            }

            element.position = element.position - vector(min_x, min_y);
            element.bounding_box.translate(vector(-min_x, -min_y));
            element.render(&mut image, &settings.font_color);
        }

        image
    }
}