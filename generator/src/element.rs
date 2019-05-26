use rusttype::{Scale, Font, Point, point, Vector, vector, PositionedGlyph};
use image::{RgbaImage, DynamicImage, Rgba, Rgb};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub position: Point<i32>,
    pub size: Vector<u32>,
    pub children: Vec<BoundingBox>,
}

impl BoundingBox {
    fn rotate(&mut self) {
        for child in self.children.iter_mut() {
            child.position = point(child.position.y,
                                   self.size.x as i32 - child.position.x - child.size.x as i32);
            child.size = vector(child.size.y, child.size.x);
        }

        self.position = point(self.position.y, self.position.x);
        self.size = vector(self.size.y, self.size.x);
    }

    pub fn translate(&mut self, offset: Vector<i32>) {
        self.position = self.position + offset;
        for child in self.children.iter_mut() {
            child.position = child.position + offset
        }
    }

    // checks for intersections between self and other WITHOUT taking
    // self.children and other.children into consideration
    fn _this_intersects(&self, other: &BoundingBox) -> bool {
        self.position.x < other.position.x + (other.size.x as i32) &&
            self.position.x + (self.size.x as i32) > other.position.x &&
            self.position.y < other.position.y + (other.size.y as i32) &&
            self.position.y + (self.size.y as i32) > other.position.y
    }

    fn intersects(&self, other: &BoundingBox) -> bool {
        // if the parents don't intersect, the children don't intersect either
        if !self._this_intersects(other) {
            return false;
        }

        for bounding_box in self.children.iter() {
            for other in other.children.iter() {
                if bounding_box._this_intersects(other) {
                    return true;
                }
            }
        }

        return false;
    }
}

pub struct Element<'a> {
    pub text: String,
    // bounding box, with position=(0, 0)
    pub bounding_box: BoundingBox,
    font_scale: f32,
    font: &'a Font<'a>,
    rotated: bool,
    // when rendering a glyph, there's a gap between the position it should be
    // rendered to and the actual glyph. _render_offset describes this gap
    _render_offset: Point<i32>,
}

impl<'a> Element<'a> {
    pub fn new(text: String, font_scale: f32, font: &'a Font<'a>) -> Self {
        let glyphs = get_glyphs(&text, font, font_scale);

        // bounding boxes, positions are NOT normalized
        let mut bounding_boxes: Vec<BoundingBox> = glyphs.iter()
            .flat_map(|g| g.pixel_bounding_box())
            .map(|bb| BoundingBox {
                position: bb.min,
                size: vector((bb.max.x - bb.min.x) as u32, (bb.max.y - bb.min.y) as u32),
                children: vec![],
            })
            .collect();

        let min_x = bounding_boxes.iter().map(|bb| bb.position.x).min().unwrap();
        let min_y = bounding_boxes.iter().map(|bb| bb.position.y).min().unwrap();
        let max_x = bounding_boxes.iter().map(|bb| bb.position.x + bb.size.x as i32).max().unwrap();
        let max_y = bounding_boxes.iter().map(|bb| bb.position.y + bb.size.y as i32).max().unwrap();

        // normalize position of bounding boxes
        // this means that the top-left corner of the first box will be at (0, 0)
        bounding_boxes.iter_mut().for_each(|bb| bb.position = bb.position - vector(min_x, min_y));

        let bounding_box = BoundingBox {
            position: point(0, 0),
            size: vector((max_x - min_x).try_into().unwrap(),
                         (max_y - min_y).try_into().unwrap()),
            children: bounding_boxes,
        };

        Element {
            text,
            bounding_box,
            font_scale,
            font,
            rotated: false,
            _render_offset: point(min_x, min_y),
        }
    }

    pub fn rotate(&mut self) {
        assert!(!self.rotated);
        self.rotated = true;
        self.bounding_box.rotate();
    }

    pub fn render(&self, font_color: &Rgb<u8>) -> RgbaImage {
        let size = self.bounding_box.size;
        let mut image = DynamicImage::new_rgba8(size.x, size.y).to_rgba();

        for glyph in get_glyphs(&self.text, self.font, self.font_scale) {
            let bb = glyph.pixel_bounding_box().unwrap();
            glyph.draw(|x, y, v| {
                let (x, y) = if self.rotated {
                    (y as i32 + bb.min.y - self._render_offset.y,
                     size.y as i32 - (x as i32 + bb.min.x - self._render_offset.x))
                } else {
                    (x as i32 + bb.min.x - self._render_offset.x,
                     y as i32 + bb.min.y - self._render_offset.y)
                };

                if v == 0.0 || x >= size.x as i32 || y >= size.y as i32 {
                    return;
                }

                image.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba {
                        data: [font_color[0], font_color[1], font_color[2], (v * 255.0) as u8],
                    },
                )
            });
        }
        image
    }
}

pub struct PositionedElement<'a> {
    pub position: Point<i32>,
    pub element: Element<'a>,
    pub bounding_box: BoundingBox,
}

impl<'a> PositionedElement<'a> {
    pub fn new(position: Point<i32>, element: Element<'a>) -> Self {
        let mut bounding_box = element.bounding_box.clone();
        bounding_box.translate(vector(position.x, position.y));
        PositionedElement { position, element, bounding_box }
    }

    pub fn render(&self, image: &mut RgbaImage, font_color: &Rgb<u8>) {
        let rendered = self.element.render(font_color);

        for x in 0..rendered.width() {
            for y in 0..rendered.height() {
                let pixel = rendered.get_pixel(x, y);
                if pixel.data[3] == 0 { continue; }

                let (img_x, img_y) = (self.position.x as u32 + x, self.position.y as u32 + y);
                image.put_pixel(img_x, img_y, *pixel);
            }
        }
    }

    pub fn collides_with(&self, other: &PositionedElement) -> bool {
        self.bounding_box.intersects(&other.bounding_box)
    }
}

fn get_glyphs<'a>(text: &String, font: &Font<'a>, uniform_scale: f32) -> Vec<PositionedGlyph<'a>> {
    let scale = Scale::uniform(uniform_scale);
    let font_metrics = font.v_metrics(scale);
    let start = point(0.0, font_metrics.ascent);
    font.layout(&text, scale, start)
        .collect()
}