// Make sure we have our third-party dependencies.
// (This is going away in future Rust, since it
// simply duplicates what's already in Cargo.toml.)
extern crate reqwest;
extern crate failure;

// Need to import a couple of things from
// the standard library
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Download file from `url` and save it to `destination`.
fn download(url: &str, destination: &Path)
            -> Result<(), failure::Error> {

    // Don't do anything if we already have the file.
    if destination.exists() {
        return Ok(())
    }

    // Otherwise, create a new file.

    // Because each of the following operations
    // can fail (returns a result type), we follow
    // them with the `?` operator. If the result
    // is an error, it will exit from the function
    // early, propagating the error upwards; if
    // the operation completed successfully, we get
    // the result instead.
    let file = File::create(destination)?;

    // We need the `mut` annotation, because
    // we're mutating (writing to) the writer.
    let mut writer = BufWriter::new(file);

    let mut response = reqwest::get(url)?;
    response.copy_to(&mut writer)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
