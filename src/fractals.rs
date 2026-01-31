//! SVG and Fractal Generation Utilities
//!
//! A collection of tools for creating SVG graphics and fractal images.
//! Includes implementations for the Koch snowflake and Mandelbrot-like fractals.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

use crate::files;
use std::cmp::min;
use std::fmt::Write;

/// Trait for objects that can be converted to SVG syntax.
pub trait Vectorizable {
    /// Converts the object to SVG syntax.
    ///
    /// # Returns
    /// A string containing the SVG representation of the object.
    fn convert_to_svg_syntax(&self) -> String;
}

/// Represents a line in SVG format.
#[derive(Clone)]
pub struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    color: String,
    thickness: u32,
}

impl Vectorizable for Line {
    fn convert_to_svg_syntax(&self) -> String {
        // Example: <line x1="0" y1="0" x2="150" y2="200" style="stroke:blue;stroke-width:2" />
        let mut line_str: String = String::new();
        line_str += "<line x1=\"";
        write!(line_str, "{}", self.x1).expect("Error in conversion (1).");
        line_str += "\" y1=\"";
        write!(line_str, "{}", self.y1).expect("Error in conversion (2).");
        line_str += "\" x2=\"";
        write!(line_str, "{}", self.x2).expect("Error in conversion (3).");
        line_str += "\" y2=\"";
        write!(line_str, "{}", self.y2).expect("Error in conversion (4).");
        line_str += "\" style=\"stroke:";
        line_str += &self.color;
        line_str += ";stroke-width:";
        write!(line_str, "{}", self.thickness).expect("Error in conversion (5).");
        line_str += "\"/>";

        line_str
    }
}

/// Creates an SVG file from vectorizable objects.
///
/// # Arguments
/// * `file_path` - Path to the output SVG file.
/// * `height` - Height of the SVG canvas.
/// * `width` - Width of the SVG canvas.
/// * `figures` - Slice of vectorizable objects to include in the SVG.
///
/// # Example
/// ```
/// let lines = vec![Line { x1: 0, y1: 0, x2: 100, y2: 100, color: "black".to_string(), thickness: 1 }];
/// let figures: Vec<Box<dyn Vectorizable>> = lines.into_iter().map(|l| Box::new(l) as Box<dyn Vectorizable>).collect();
/// create_svg_file(&String::from("output.svg"), 200, 200, &figures);
/// ```
pub fn create_svg_file(
    file_path: &String,
    height: u32,
    width: u32,
    figures: &[Box<dyn Vectorizable>],
) {
    let mut content_vec: Vec<String> = Vec::new();

    // First line: SVG header
    let mut line: String = String::new();
    line += "<svg height=\"";
    write!(line, "{}", height).expect("Error in conversion (1).");
    line += "\" width=\"";
    write!(line, "{}", width).expect("Error in conversion (1).");
    line += "\" xmlns=\"http://www.w3.org/2000/svg\">";

    content_vec.push(line);

    for figure in figures.iter() {
        let line: String = figure.convert_to_svg_syntax();
        content_vec.push(line);
    }

    // Last line: Close SVG tag
    let line: String = "</svg>".to_string();
    content_vec.push(line);

    files::write_text_file_lines(file_path, &content_vec);
}

/// Creates an SVG file from a collection of lines.
///
/// # Arguments
/// * `file_path` - Path to the output SVG file.
/// * `height` - Height of the SVG canvas.
/// * `width` - Width of the SVG canvas.
/// * `lines` - Vector of Line objects to include in the SVG.
///
/// # Example
/// ```
/// let lines = vec![Line { x1: 0, y1: 0, x2: 100, y2: 100, color: "black".to_string(), thickness: 1 }];
/// create_svg_file_from_lines(&String::from("output.svg"), 200, 200, lines);
/// ```
pub fn create_svg_file_from_lines(file_path: &String, height: u32, width: u32, lines: Vec<Line>) {
    let mut figures: Vec<Box<dyn Vectorizable>> = Vec::new();
    for line in lines {
        let figure: Box<dyn Vectorizable> = Box::new(line);
        figures.push(figure);
    }
    create_svg_file(file_path, height, width, &figures);
}

/// Recursively generates the Koch snowflake fractal.
///
/// # Arguments
/// * `lines` - Current set of lines.
/// * `n_iter` - Number of remaining iterations.
///
/// # Returns
/// A vector of lines representing the Koch snowflake at the specified iteration.
///
/// # Example
/// ```
/// let initial_lines = vec![Line { x1: 0, y1: 0, x2: 100, y2: 0, color: "blue".to_string(), thickness: 1 }];
/// let snowflake = koch_snowflake_recursive(&initial_lines, 3);
/// ```
pub fn koch_snowflake_recursive(lines: &Vec<Line>, n_iter: u32) -> Vec<Line> {
    let sqrt_3_over_2 = f32::sqrt(3.) / 2.;
    if n_iter == 0 {
        return lines.clone();
    }

    let mut returned_lines = Vec::new();

    for line in lines {
        // Transform each segment into 4 segments
        let x1 = line.x1;
        let y1 = line.y1;
        let x2 = (2. / 3. * (line.x1 as f32) + 1. / 3. * (line.x2 as f32)) as u32;
        let y2 = (2. / 3. * (line.y1 as f32) + 1. / 3. * (line.y2 as f32)) as u32;

        let x4 = (1. / 3. * (line.x1 as f32) + 2. / 3. * (line.x2 as f32)) as u32;
        let y4 = (1. / 3. * (line.y1 as f32) + 2. / 3. * (line.y2 as f32)) as u32;
        let x5 = line.x2;
        let y5 = line.y2;

        let dx24: i32 = (sqrt_3_over_2 * ((x4 as i32 - x2 as i32) as f32)) as i32;
        let dy24: i32 = (sqrt_3_over_2 * ((y4 as i32 - y2 as i32) as f32)) as i32;
        let mx24 = (x4 + x2) / 2;
        let my24 = (y4 + y2) / 2;
        let x3 = (mx24 as i32 + dy24) as u32;
        let y3 = (my24 as i32 - dx24) as u32;

        let color = line.color.clone();
        let thickness = line.thickness;
        returned_lines.push(Line {
            x1,
            y1,
            x2,
            y2,
            color: color.clone(),
            thickness,
        });
        returned_lines.push(Line {
            x1: x2,
            y1: y2,
            x2: x3,
            y2: y3,
            color: color.clone(),
            thickness,
        });
        returned_lines.push(Line {
            x1: x3,
            y1: y3,
            x2: x4,
            y2: y4,
            color: color.clone(),
            thickness,
        });
        returned_lines.push(Line {
            x1: x4,
            y1: y4,
            x2: x5,
            y2: y5,
            color: color.clone(),
            thickness,
        });
    }

    koch_snowflake_recursive(&returned_lines, n_iter - 1)
}

/// Generates a Koch snowflake fractal.
///
/// # Arguments
/// * `height` - Height of the SVG canvas.
/// * `width` - Width of the SVG canvas.
/// * `n_iter` - Number of iterations.
///
/// # Returns
/// A vector of lines representing the Koch snowflake.
///
/// # Example
/// ```
/// let snowflake = koch_snowflake(500, 500, 3);
/// ```
pub fn koch_snowflake(height: u32, width: u32, n_iter: u32) -> Vec<Line> {
    let size_ratio: f32 = 0.8;
    let sqrt_3 = f32::sqrt(3.);
    let color: String = "blue".to_string();
    let thickness: u32 = 3;
    let mut lines: Vec<Line> = Vec::new();

    let m = min(height, width);
    let length = (size_ratio * (m as f32)) as u32;
    let x1 = width / 2 - length / 2;
    let y1 = height / 2 - (length as f32 * sqrt_3 / 6.) as u32;
    let x2 = width / 2 + length / 2;
    let y2 = y1;
    let y3 = height / 2 + (length as f32 * sqrt_3 * 2. / 6.) as u32;
    let x3 = width / 2;
    lines.push(Line {
        x1,
        y1,
        x2,
        y2,
        color: color.clone(),
        thickness,
    });

    lines.push(Line {
        x1: x2,
        y1: y2,
        x2: x3,
        y2: y3,
        color: color.clone(),
        thickness,
    });
    lines.push(Line {
        x1: x3,
        y1: y3,
        x2: x1,
        y2: y1,
        color: color.clone(),
        thickness,
    });

    koch_snowflake_recursive(&lines, n_iter)
}

/// Calculates a fractal pattern internally.
///
/// # Arguments
/// * `x_min`, `x_max` - X-axis range.
/// * `x_n_step` - Number of steps along X-axis.
/// * `y_min`, `y_max` - Y-axis range.
/// * `y_n_step` - Number of steps along Y-axis.
/// * `max_n_iter` - Maximum number of iterations.
/// * `x_fractal`, `y_fractal` - Fractal parameters.
///
/// # Returns
/// A vector of values representing the fractal pattern.
#[allow(clippy::too_many_arguments)]
fn calculate_internal_fractal(
    x_min: f64,
    x_max: f64,
    x_n_step: u32,
    y_min: f64,
    y_max: f64,
    y_n_step: u32,
    max_n_iter: usize,
    x_fractal: f64,
    y_fractal: f64,
) -> Vec<f64> {
    let mut pixels: Vec<f64> = Vec::new();

    let x_step = (x_max - x_min) / ((x_n_step - 1) as f64);
    let y_step = (y_max - y_min) / ((y_n_step - 1) as f64);

    for y_index in 0..y_n_step {
        let y_current = y_min + y_step * (y_index as f64);

        for x_index in 0..x_n_step {
            let x_current = x_min + x_step * (x_index as f64);

            let mut x_n: f64 = x_current;
            let mut y_n: f64 = y_current;
            let mut norm: f64 = 0.;
            for _ in 0..max_n_iter {
                let x_np1: f64 = x_n * x_n - y_n * y_n + x_fractal;
                let y_np1: f64 = 2. * x_n * y_n + y_fractal;
                x_n = x_np1;
                y_n = y_np1;
                norm = f64::sqrt(x_n * x_n + y_n * y_n);
                if norm >= 2. {
                    break;
                }
            }
            norm = (2. - norm) / 2.;
            if norm < 0. {
                norm = 0.;
            }
            pixels.push(norm);
        }
    }

    pixels
}

/// Converts a hexadecimal string with space-separated bytes to a byte vector.
///
/// # Arguments
/// * `hex_string` - String containing space-separated hexadecimal bytes.
///
/// # Returns
/// A vector of bytes converted from the hexadecimal string.
///
/// # Example
/// ```
/// let bytes = convert_hex_string_to_vec("42 4d 36 10 0e 00");
/// ```
fn convert_hex_string_to_vec(hex_string: &str) -> Vec<u8> {
    let mut returned_bytes: Vec<u8> = Vec::new();
    let hex_bytes = hex_string.split_whitespace().collect::<Vec<_>>();
    for hex_byte in hex_bytes {
        let v = u8::from_str_radix(hex_byte, 16)
            .expect("Error with u8::from_str_radix(): Could not convert.");
        returned_bytes.push(v);
    }

    returned_bytes
}

/// Calculates a fractal image and writes it to a BMP file.
///
/// # Arguments
/// * `x_fractal`, `y_fractal` - Fractal parameters.
/// * `bmp_file_path` - Path to the output BMP file.
///
/// # Example
/// ```
/// calculate_fractal_and_write_bmp(-0.7, 0.27015, &String::from("fractal.bmp"));
/// ```
pub fn calculate_fractal_and_write_bmp(x_fractal: f64, y_fractal: f64, bmp_file_path: &String) {
    let mut bmp_bytes: Vec<u8> = Vec::new();

    let x_min = -1.;
    let x_max = 1.;
    let y_min = -1.;
    let y_max = 1.;

    let max_n_iter: usize = 50;

    // BMP header (640x480) - Hardcoded resolution (Fixed in the BMP header)
    // TODO: Find a way to write a matrix image without external libraries,
    //       allowing custom parameters (resolution) etc.
    let height = 640;
    let width = 480;
    let bmp_header1 = "42 4d 36 10 0e 00 00 00 00 00 36 00 00 00 28 00";
    let bmp_header2 = "00 00 80 02 00 00 e0 01 00 00 01 00 18 00 00 00";
    let bmp_header3 = "00 00 00 10 0e 00 d7 0d 00 00 d7 0d 00 00 00 00";
    let bmp_header4 = "00 00 00 00 00 00";
    bmp_bytes.append(&mut convert_hex_string_to_vec(bmp_header1));
    bmp_bytes.append(&mut convert_hex_string_to_vec(bmp_header2));
    bmp_bytes.append(&mut convert_hex_string_to_vec(bmp_header3));
    bmp_bytes.append(&mut convert_hex_string_to_vec(bmp_header4));

    let img_pixels: Vec<f64> = calculate_internal_fractal(
        x_min, x_max, width, y_min, y_max, height, max_n_iter, x_fractal, y_fractal,
    );

    for x_index in 0..width {
        for y_index in 0..height {
            let pixel_index = (y_index * width + x_index) as usize;
            let pixel: f64 = img_pixels[pixel_index];
            let intensity: u8 = (255. * pixel) as u8;
            bmp_bytes.push(intensity);
            bmp_bytes.push(0);
            bmp_bytes.push(0);
        }
    }

    files::write_binary_file(bmp_file_path, &bmp_bytes);
}
