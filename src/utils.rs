use std::{fs::File, io::{BufReader, Read}, error::Error};
use eframe::App;

use crate::errors::AppError;

pub struct  Function (pub Box< dyn FnMut(f64)->f64>);
impl  Function {
    pub fn new(f: impl Fn(f64) -> f64 + 'static)->Self{
        Self(Box::new(f))
        
    }
}

pub fn read_points_from_file(filename: &str) -> Result<Vec<[f64; 2]>, AppError> {
    let mut  file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(AppError::IOError),
    };
    let mut contents = String::new();
    if !file.read_to_string(&mut contents).is_ok(){
        return Err(AppError::IOError)
    }

    let mut points: Vec<[f64; 2]> = Vec::new();
    for line in contents.lines() {
        let (x, y) = line
           .split_once(' ')
            .ok_or(AppError::InvalidFormat)?;

        let x_fromstr = x.parse::<f64>().map_err(|_| AppError::InvalidFormat)?;
        let y_fromstr = y.parse::<f64>().map_err(|_| AppError::InvalidFormat)?;
        points.push([x_fromstr, y_fromstr]);
    }

    Ok(points)
}