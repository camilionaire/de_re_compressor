use itertools::Itertools;
use std::{collections::HashSet, env, fs};

fn main() {
    //    println!("Hello, world!");
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("to run:\ncargo run <compress/decompress> <filename>");
        std::process::exit(1);
    }

    let contents =
        fs::read_to_string(args[1].clone()).expect("something went wrong with the file name...");

    if args[0] == "compress" {
        compressor(contents);
    } else if args[0] == "decompress" {
        decompressor(contents);
    } else {
        println!("to run:\ncargo run <compress/decompress> <filename>");
        std::process::exit(1);
    }
}

fn compressor(input: String) {
    println!("Compression starting...");

    let uniq: Vec<&str> = input
        .split_whitespace()
        .collect::<HashSet<_>>()
        .into_iter()
        // I got how to find the sorted without doing it in place as mutable from:
        // https://stackoverflow.com/questions/54701548/
        // how-can-i-sort-an-iterator-without-putting-it-all-in-a-vector
        .sorted()
        .collect();

    let updated: String = input
        .lines()
        .map(|n| {
            n.split_terminator('\t')
                .map(|t| {
                    t.split_terminator(' ')
                        .map(|s| {
                            if !s.is_empty() {
                                // got how to find the position from:
                                // https://stackoverflow.com/questions/30558246/
                                // how-do-i-find-the-index-of-an-element-in-an-array-vector-or-slice
                                uniq.iter().position(|c| c == &s).unwrap().to_string()
                            } else {
                                s.to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let printable = uniq.join(" ");
    let _ = fs::write("out_enc.txt", format!("{printable}\n{updated}"));
    println!("File compressed and can be found at 'out_enc.txt'");
}

fn decompressor(input: String) {
    println!("Decompression starting...");

    let dict: Vec<&str> = input.lines().next().unwrap().split_whitespace().collect();

    let updated: String = input
        .lines()
        .skip(1)
        .map(|n| {
            n.split_terminator('\t')
                .map(|t| {
                    t.split_terminator(' ')
                        .map(|s| {
                            if !s.is_empty() {
                                dict[s.to_string().parse::<usize>().unwrap()].to_string()
                            } else {
                                s.to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let _ = fs::write("out_dec.txt", updated);
    println!("File decompressed and can be found at 'out_dec'.txt");
}