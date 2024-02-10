use std::{io::Read, path::PathBuf};

use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

const COLOR_MONOKAI_BACKGROUND: image::Rgba<u8> = image::Rgba([46, 46, 46, 255]);
const COLOR_MONOKAI_YELLOW: image::Rgba<u8> = image::Rgba([229, 181, 103, 255]);
const COLOR_MONOKAI_GREEN: image::Rgba<u8> = image::Rgba([180, 210, 115, 255]);
const COLOR_MONOKAI_ORANGE: image::Rgba<u8> = image::Rgba([232, 125, 62, 255]);
const COLOR_MONOKAI_PURPLE: image::Rgba<u8> = image::Rgba([158, 134, 200, 255]);
const COLOR_MONOKAI_PINK: image::Rgba<u8> = image::Rgba([176, 82, 121, 255]);
const COLOR_MONOKAI_BLUE: image::Rgba<u8> = image::Rgba([108, 153, 187, 255]);
const COLORS: &[image::Rgba<u8>] = &[
    COLOR_MONOKAI_YELLOW,
    COLOR_MONOKAI_GREEN,
    COLOR_MONOKAI_ORANGE,
    COLOR_MONOKAI_PURPLE,
    COLOR_MONOKAI_PINK,
    COLOR_MONOKAI_BLUE,
];

const ICON_SIZE: u32 = 64;

// -----------------------------------------------------------------------------
//  - Configuration -
// -----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Dimension {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Modifier {
    Square,
    Colorized,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub dimensions: Vec<Dimension>,
    pub modifiers: Vec<Modifier>,
    pub font: PathBuf,
    pub icons: Vec<String>,
}

fn draw_font(font: Font, size: f32, color: Rgba<u8>, text: &str) -> DynamicImage {
    // -----------------------------------------------------------------------------
    //  - get the width of the font icon -
    // -----------------------------------------------------------------------------
    let scale = Scale::uniform(size);
    let metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, 0.0 + metrics.ascent))
        .collect();
    let glyph_height = (metrics.ascent - metrics.descent).ceil() as u32;
    let glyph_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // -----------------------------------------------------------------------------
    //  - create an image and draw the text (glyph) on it -
    // -----------------------------------------------------------------------------
    let mut img = DynamicImage::new_rgba8(glyph_width, glyph_height);
    draw_text_mut(&mut img, color, 0, 0, scale, &font, text);
    img
}

pub fn draw_circle(
    background: Rgba<u8>,
    foreground: Rgba<u8>,
    size: f32,
    font: Font,
    text: &str,
) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(ICON_SIZE, ICON_SIZE);
    // -----------------------------------------------------------------------------
    //  - draw background circle shape -
    // -----------------------------------------------------------------------------
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, background);
        }
    }

    // -----------------------------------------------------------------------------
    //  - add icon overlay to circle -
    // -----------------------------------------------------------------------------
    let font = draw_font(font, size, foreground, text);
    image::imageops::overlay(
        &mut image,
        &font,
        (ICON_SIZE / 2 - font.width() / 2).into(),
        (ICON_SIZE / 2 - font.height() / 2).into(),
    );
    image
}

#[derive(Debug, clap::Parser)]
pub struct Arguments {
    #[arg(long)]
    pub config: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
}

pub fn main() {
    let args = Arguments::parse();

    let config = std::fs::read_to_string(args.config).expect("load config file");
    let config: Configuration = serde_yaml::from_str(&config).expect("deserialize config file");

    // -----------------------------------------------------------------------------
    //  - load the requested font -
    // -----------------------------------------------------------------------------
    let f = std::fs::File::open(config.font).unwrap();
    let mut reader = std::io::BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let font = Font::try_from_vec(buffer).unwrap();

    for dimension in config.dimensions {
        let mut img = DynamicImage::new_rgba8(dimension.width, dimension.height);
        let (width, height) = img.dimensions();

        // -----------------------------------------------------------------------------
        //  - set background color of image -
        // -----------------------------------------------------------------------------
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x, y, COLOR_MONOKAI_BACKGROUND);
            }
        }

        // -----------------------------------------------------------------------------
        //  - try to place an icon on the image -
        // -----------------------------------------------------------------------------

        let rows = (width - ICON_SIZE) / ICON_SIZE;
        let columns = (height - ICON_SIZE) / ICON_SIZE;

        let mut cells: Vec<(u32, u32)> = Vec::new();

        let mut rng = fastrand::Rng::default();
        for _ in 0..10 {
            let mut value = (
                rng.u32(1..rows) * ICON_SIZE,
                rng.u32(1..columns) * ICON_SIZE,
            );
            // we want to ensure that we have the configured number of icons placed, we
            // have to repeat this until we find a value that wasn't already generated ...
            loop {
                if !cells.contains(&value) {
                    cells.push(value);
                    break;
                } else {
                    value = (
                        rng.u32(1..rows) * ICON_SIZE,
                        rng.u32(1..columns) * ICON_SIZE,
                    );
                }
            }

            let color = rng.choice(COLORS).unwrap_or(&COLOR_MONOKAI_GREEN);
            let circle = draw_circle(
                *color,
                COLOR_MONOKAI_BACKGROUND,
                42.0,
                font.clone(),
                &config.icons[rng.usize(..config.icons.len())],
            );

            let x = value.0;
            let y = value.1;

            image::imageops::overlay(&mut img, &circle, x.into(), y.into());
        }

        let _ = img.save(args.output.join(format!(
            "image_{}x{}.png",
            dimension.width, dimension.height
        )));
    }
}
