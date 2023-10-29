use std::{fs::DirEntry, io::Error, time::Instant};

use image::io::Reader;

#[link(name = "wooting-rgb-sdk")]
extern {
    fn wooting_usb_find_keyboard() -> bool;
    fn wooting_rgb_array_set_full(colors_buffer: *const u8) -> bool;
    fn wooting_rgb_reset() -> bool;
    fn wooting_rgb_array_update_keyboard() -> bool;
}

/// How many columns are there?
const COLUMNS: usize = 21;
/// How many rows are there?
const ROWS: usize = 6;
/// How many components are there in a color?
const COMPONENTS: usize = 3;

#[derive(Debug, Copy, Clone)]
struct KeyCode(u8, u8);

fn set_full(array: &[(KeyCode, (u8, u8, u8))]) -> bool {
    let mut flattened: [u8; COMPONENTS * COLUMNS * ROWS] = [0; COMPONENTS * COLUMNS * ROWS];
    for (key, (red, green, blue)) in array {
        let KeyCode(column, row) = key;
        let index: usize =
            (*row as usize) * (COLUMNS * COMPONENTS) + (*column as usize) * COMPONENTS;
        flattened[index] = *red;
        flattened[index + 1] = *green;
        flattened[index + 2] = *blue;
    }
    unsafe { wooting_rgb_array_set_full(flattened.as_ptr()) }
}

fn main() {
    let keyboard_found = unsafe { wooting_usb_find_keyboard() };

    println!(
        "Keyboard connected? {}",
        keyboard_found
    );

    // list files in data/resized
    let files: Vec<Result<DirEntry, Error>> = std::fs::read_dir("data/resized").unwrap().collect();

    unsafe { wooting_rgb_reset(); }

    let mut files = files.iter().map(|f| f.as_ref().unwrap().path().to_str().unwrap().to_string()).collect::<Vec<String>>();

    files.sort();

    for i in files {
        let start = Instant::now();
        let image = Reader::open(i.clone()).unwrap().decode().unwrap().to_rgb8();

        let mut keys = Vec::new();

        for (x, y, pixel) in image.enumerate_pixels() {
            let key = KeyCode(x as u8, y as u8);
            let color: (u8,u8,u8) = (pixel[0], pixel[1], pixel[2]);

            keys.push((key, color));
        }

        // dbg!(&keys);

        set_full(&keys);
        unsafe { wooting_rgb_array_update_keyboard(); }
        let elapsed = start.elapsed();
        let target_duration = std::time::Duration::from_micros(1_000_000 / 30);
        if elapsed < target_duration {
            println!("Sleeping, took {:?} {:?} {}", elapsed, target_duration, i);
            std::thread::sleep(target_duration - elapsed);
        }
    }

    unsafe { wooting_rgb_reset(); }
}
