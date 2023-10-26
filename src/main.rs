use image::{DynamicImage, GenericImageView};

const CHAR_MAP_LEN: usize = 92;
static CHAR_MAP: [char; CHAR_MAP_LEN] = [
    ' ', '`', '.', '-', '\'', ':', '_', ',', '^', '=', ';', '>', '<', '+', '!', 'r', 'c', '*', '/',
    'z', '?', 's', 'L', 'T', 'v', ')', 'J', '7', '(', '|', 'F', 'i', '{', 'C', '}', 'f', 'I', '3',
    '1', 't', 'l', 'u', '[', 'n', 'e', 'o', 'Z', '5', 'Y', 'x', 'j', 'y', 'a', ']', '2', 'E', 'S',
    'w', 'q', 'k', 'P', '6', 'h', '9', 'd', '4', 'V', 'p', 'O', 'G', 'b', 'U', 'A', 'K', 'X', 'H',
    'm', '8', 'R', 'D', '#', '$', 'B', 'g', '0', 'M', 'N', 'W', 'Q', '%', '&', '@',
];

fn main() {
    let filepath = "danielle.jpg";
    let img_scale = 16;

    let img = match image::open(filepath) {
        Ok(img) => img,
        Err(e) => {
            panic!(
                "An error occured when attempting to read the file '{}':\n\t{}",
                filepath, e
            );
        }
    };

    let ascii_string = match turn_image_into_ascii(img, img_scale) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error occured when converting image to ascii:\n\t{}", e);
        }
    };

    println!("{}", ascii_string)
}

fn get_ascii_for_intensity(intensity: f64) -> char {
    let idx = intensity / 2.8;
    CHAR_MAP[idx as usize]
}

fn turn_image_into_ascii(img: DynamicImage, scale: u32) -> Result<String, anyhow::Error> {
    let (width, height) = img.dimensions();
    let mut ascii_result: Vec<char> = Vec::with_capacity(((width * (height * 2)) * scale) as usize);

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let px = img.get_pixel(x, y);

                // Add RGB values to get intensity
                let mut intensity = (px[0] as f64 + px[1] as f64 + px[2] as f64) / 3.0;

                // If alpha is 0, then make it transparent
                if px[3] == 0 {
                    intensity = 0.0;
                }

                let ascii_char = get_ascii_for_intensity(intensity);
                ascii_result.push(ascii_char);
            }
        }

        if y % (scale * 2) == 0 {
            ascii_result.push('\n');
        }
    }

    Ok(String::from_iter(ascii_result))
}
