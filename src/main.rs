mod pff_archive;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let terrains_pff_path = Path::new("data/Terrains.pff");
    let display = terrains_pff_path.display();

    let mut file = match File::open(&terrains_pff_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    println!("Succesfully opened file!");
}
