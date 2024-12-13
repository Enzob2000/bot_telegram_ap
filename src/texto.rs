use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::{collections::VecDeque, io::stdin};

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use std::path::Path;

const detection_model_path: &[u8] = include_bytes!("text-detection.rten");
const rec_model_path: &[u8] = include_bytes!("text-recognition.rten");

pub fn lector() -> Result<&'static str, Box<dyn Error>> {
    let detection_model = Model::load_static_slice(&detection_model_path)?;
    let recognition_model = Model::load_static_slice(&rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;


    let img = image::open("rust-book.jpg").map(|image| image.into_rgb8())?;

    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    let tex = engine.get_text(&ocr_input).unwrap();
    println!("{}", tex);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if tex.contains("Agotado") {
        return Ok("Agotado");
    }

    if tex.contains("numero") {
        return Ok("numero");
    }

    Ok("")
}
