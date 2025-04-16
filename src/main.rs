//! Main executable for the RNASEQ Toolkit.

use std::{
    env::args,
    fs::read_to_string,
};

use rnaseq_sam::Transcriptome;

fn main() {
    println!("RNASEQ Toolkit");

    let args = args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("File input required");
        return;
    }

    let fname = args[1].clone();

    let raw = read_to_string(fname).expect("Couldn't open file");

    let transcriptome = Transcriptome::from(&raw);
    
    // TODO do something with the transcriptome
    println!("{:#?}", transcriptome);
}