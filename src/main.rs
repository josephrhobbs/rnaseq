//! Main executable for the RNASEQ Toolkit.

use std::{
    env::args,
    fs::read_to_string,
};

use rnaseq_gff::FeatureList;

use rnaseq_sam::Transcriptome;

fn main() {
    println!("RNASEQ Toolkit");

    let args = args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("File inputs required: rnaseq [SAM] [GFF3]");
        return;
    }

    let sam_fname = args[1].clone();
    let raw = read_to_string(sam_fname).expect("Couldn't open SAM file");
    let transcriptome = Transcriptome::from(&raw);

    let gff3_fname = args[2].clone();
    let raw = read_to_string(gff3_fname).expect("Couldn't open GFF3 file");
    let feature_list = FeatureList::from(&raw);
    
    // TODO do something with the transcriptome
    // println!("{:#?}", transcriptome);
    println!("{:#?}", feature_list);
}