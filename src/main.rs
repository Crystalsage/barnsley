use std::fs::File;
use std::io::Write;

use rand::Rng;

static FILE_PATH: &str = "output.ppm";

type Color32 = i64;
type Image = Vec<Vec<Color32>>;

const MAX_ITERATIONS: usize = 200_000usize;

const WIDTH: i64 = 1920;
const HEIGHT: i64 = 1080;

const COLOR_BACKGROUND: Color32 = 0xFF1D2021;
const COLOR_LEAF: Color32 = 0xFFB8BB26;

fn fill_background(image: &mut Image, color: Color32) {
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            image[y][x] = color;
        }
    }
}

// Barnsley's leaf algorithm
fn draw_leaf(image: &mut Image) {
    let mut rng = rand::thread_rng();
    let mut x = 0.;
    let mut y = 0.;

    for _ in 0..MAX_ITERATIONS {
            let r = rng.gen::<f32>();
            let cx: f64;
            let cy: f64;

            if r <= 0.01 {
                cx = 0f64;
                cy = 0.16 * y as f64;
            } else if r <= 0.08 {
                cx = 0.2 * x as f64 - 0.26 * y as f64;
                cy = 0.23 * x as f64 + 0.22 * y as f64 + 1.6;
            } else if r <= 0.15 {
                cx = -0.15 * x as f64 + 0.28 * y as f64;
                cy = 0.26 * x as f64 + 0.26 * y as f64 + 0.44;
            } else {
                cx = 0.85 * x as f64 + 0.04 * y as f64;
                cy = -0.04 * x as f64 + 0.85 * y as f64 + 1.6;
            }

            x = cx;
            y = cy;

            let x_coord = ((WIDTH as f64) / 2. + x * (WIDTH as f64) / 11.).round() as usize;
            let y_coord = ((HEIGHT as f64) - y * (HEIGHT as f64) / 11.).round() as usize;

            image[y_coord as usize][x_coord as usize] = COLOR_LEAF;
    }
}

fn write_image_to_file(image: Image) -> std::io::Result<()> {
   let mut file = File::create(FILE_PATH)?;
   file.write_all(format!("P6\n{} {} 255\n", WIDTH, HEIGHT).as_bytes())?;

   let mut all_bytes: Vec<u8> = Vec::new();

   for y in 0..HEIGHT as usize {
       for x in 0..WIDTH as usize {
           let pixel: i64 = image[y][x];

           // Extract red component
           all_bytes.push(((pixel&0x0000FF) >> 8*0) as u8);

           // Extract green component
           all_bytes.push(((pixel&0x00FF00) >> 8*1) as u8);

           // Extract blue component
           all_bytes.push(((pixel&0xFF0000) >> 8*2) as u8);
       }
   }

   file.write_all(&all_bytes).unwrap();

   Ok(())
}

fn main() {
    let mut image: Image = vec![vec![0_i64; WIDTH as usize]; HEIGHT as usize];

    fill_background(&mut image, COLOR_BACKGROUND);
    draw_leaf(&mut image);
    let _ = write_image_to_file(image);
}
