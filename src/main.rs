//! Main executable for the RNASEQ Toolkit.

use std::{
    collections::HashMap,
    env::args,
    fs::read_to_string,
    time::Instant,
};

use rnaseq_gff::{
    FeatureList,
    FeatureType,
};

use rnaseq_sam::Transcriptome;

fn main() {
    println!("RNASEQ Toolkit");
    println!();

    let args = args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("File inputs required: rnaseq [SAM] [GFF3]");
        return;
    }

    // Parse SAM
    let sam_fname = args[1].clone();
    let start = Instant::now();
    let raw = read_to_string(sam_fname).expect("Couldn't open SAM file");
    let transcriptome = Transcriptome::from(&raw).unwrap();
    let end = start.elapsed();
    println!("Parsed {} alignments in {:.3} ms", transcriptome.alignments.len(), end.as_micros() as f64 / 1000.0);

    // Parse GFF3
    let gff3_fname = args[2].clone();
    let start = Instant::now();
    let raw = read_to_string(gff3_fname).expect("Couldn't open GFF3 file");
    let feature_list = FeatureList::from(&raw, |f| f.feature_type == Some (FeatureType::Gene)).unwrap();
    let end = start.elapsed();
    println!("Parsed {} features in {:.3} ms", feature_list.features.len(), end.as_micros() as f64 / 1000.0);
    println!();
    
    // Determine gene expression
    let start = Instant::now();
    let mut expressions = HashMap::<String, usize>::new();
    for alignment in &transcriptome.alignments {
        if let Some (apos) = alignment.pos {
            // Indices for bisection search
            let mut first = 0;
            let mut second = feature_list.features.len() - 1;

            while second > first + 1 {
                // Check middle
                let middle = (first + second) / 2;
                let fstart = feature_list.features[middle].start.unwrap();

                if apos < fstart {
                    second = middle;
                } else if fstart < apos {
                    first = middle;
                } else {
                    break;
                }
            }

            let fend = feature_list.features[first].end.unwrap();

            if apos <= fend {
                let name = feature_list.features[first].name.clone().unwrap_or("<UNK>".to_string());

                if let Some (v) = expressions.get(&name) {
                    expressions.insert(name, v+1);
                } else {
                    expressions.insert(name, 1);
                }
            }            
        }
    }
    let end = start.elapsed();

    println!("{:#?}", expressions);

    println!("Calculated expression of {} genes in {:.3} ms", expressions.len(), end.as_micros() as f64 / 1000.0);
}