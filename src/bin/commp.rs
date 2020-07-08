// Generate a Filecoin CommP for a file
// Usage: commp <path to file> [fp]
// specify "fp" to run through filecoin_proofs

use std::env;
use std::fs::File;

use flexi_logger::Logger;
use hex;

fn usage() {
    print!("Usage: commp [-fp|-sp|-spl] <file>\n");
}

fn to_mb(size: u64) -> String {
    let r = (((size as f64) / 1024.0 / 1024.0) * 100.0).round() / 100.0;
    format!("{} Mb", r)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::with_str("info").start().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
        return Err(From::from("Not enough arguments".to_string()));
    }
    let filename = &args[2];
    let mut file = File::open(filename)?;
    let file_size = file.metadata().unwrap().len();

    let commp: commp::CommP;

    if &args[1] == "-fp" {
        commp = commp::generate_commp_filecoin_proofs(&mut file, file_size).unwrap();
    } else if &args[1] == "-sp" {
        commp = commp::generate_commp_storage_proofs(&mut file, file_size).unwrap();
    } else if &args[1] == "-spl" {
        commp = commp::generate_commp_storage_proofs_mem(&mut file, file_size, false).unwrap();
    } else if &args[1] == "-splm" {
        commp = commp::generate_commp_storage_proofs_mem(&mut file, file_size, true).unwrap();
    } else {
        usage();
        return Err(From::from("Supply one of -fp (filecoin-proofs), -sp (storage-proofs) or -spl (storage-proofs local / reimplemented)".to_string()));
    }

    print!(
        "{}:\n\tSize: {}\n\tPadded Size: {}\n\tPiece Size: {}\n\tCommP {}\n",
        filename,
        to_mb(file_size),
        to_mb(commp.padded_size),
        to_mb(commp.piece_size),
        hex::encode(commp.bytes)
    );

    Ok(())
}
