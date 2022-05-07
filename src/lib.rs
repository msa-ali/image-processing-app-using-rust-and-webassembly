use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log_1;
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

fn log(value: &str) {
    log_1(&value.into());
}

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // log(&encoded_file.into());
    log("Grayscale called");
    let base64_to_vector = decode(encoded_file).unwrap();
    log("Image Decoded");

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log("Image loaded");
    image = image.grayscale(); 
    log("Grayscale effect has been applied");

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log("New Image written");

    let encoded_img = encode(&buffer);

    let data_url = format!("data:image/png;base64,{}", encoded_img);

    return data_url;
}