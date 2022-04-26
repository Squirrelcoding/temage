// Import dependencies
use anyhow::{bail, Context, Result};
use colored::*;
use std::{ffi::OsStr, fs, path::Path};

// This is where the magic happens! This file reads the contents of the given path and displays an image in the terminal

pub fn read(file_name: &str) -> Result<()> {
    // Gets the extension of the file, and panics if it isnt .tm
    if get_extension_from_filename(file_name).unwrap() != "tm" {
        bail!(format!("Invalid file: {}", file_name).bold().red());
    }

    // Read the contents of the file into a buffer
    let buf = fs::read(file_name)
        .with_context(|| format!("Failed to read file: {}", file_name).bold().red())?;

    // The first byte of the buffer contains the width of the image (height isnt necessary to know btw)
    let width = buf[0];

    // Iterate through every byte in the buffer.
    for (i, item) in buf.iter().enumerate().skip(1) {
        // If i is a multiple of the width, then make a new line
        if (i - 1) % (width as usize) == 0 {
            println!();
        }

        // Match expression to determine the color of the "pixel" (really just an invisible character)
        match item {
            // Black pixel
            0 => {
                print!("{}", " ".on_black());
            }

            // Red pixel
            1 => {
                print!("{}", " ".on_red());
            }

            // Green pixel
            2 => {
                print!("{}", " ".on_green());
            }

            // Yellow pixel
            3 => {
                print!("{}", " ".on_yellow());
            }

            // Blue pixel
            4 => {
                print!("{}", " ".on_blue());
            }

            // Purple pixel
            5 => {
                print!("{}", " ".on_purple());
            }

            // Cyan pixel
            6 => {
                print!("{}", " ".on_cyan());
            }

            // White pixel
            7 => {
                print!("{}", " ".on_white());
            }

            // Throw error if an invalid pixel is encountred
            _ => {
                bail!("An invalid value was found in the file.".bold().red())
            }
        }
    }
    Ok(())
}

// Gets the extension of a file
// e.g file.tm returns .tm
fn get_extension_from_filename(file_name: &str) -> Option<&str> {
    Path::new(file_name).extension().and_then(OsStr::to_str)
}
