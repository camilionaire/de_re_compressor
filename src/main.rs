// need intertools to sort the dictionary and return sorted copy
// sort in the std library sorts in place and requires mutable variables.
use itertools::Itertools;
use std::{collections::HashSet, env, fs};

fn main() {
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

    // this creates our dictionary.  it takes the inputs, collects the hashset
    // creating the individual words, and creates an iterable from that sorts it
    // and recollects it as a vector.
    let uniq: Vec<&str> = input
        .split_whitespace()
        .collect::<HashSet<_>>()
        .into_iter()
        // I got how to find the sorted without doing it in place as mutable from:
        // https://stackoverflow.com/questions/54701548/
        // how-can-i-sort-an-iterator-without-putting-it-all-in-a-vector
        .sorted()
        .collect();

    // turns the input into an iterable based with each element being a line
    // that in turn is turned into an interator based on tabs with each element in turn
    // being turned into an iterable based on spaces
    // each of those, if not empty, get's turned into the position in the dictionary vector.
    // those all get connected back up the line with the appropriate whitespace ' ', '\t', '\n'
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
    // let _ = fs::write("files/out_enc.txt", format!("{printable}\n{updated}"));
    println!("File compressed and can be found here:");//at 'files/out_enc.txt'");
    println!("{}\n{}", printable, updated);
}

fn decompressor(input: String) {
    println!("Decompression starting...");

    // collects the first line of the input and turns it into a vector.
    let dict: Vec<&str> = input.lines().next().unwrap().split_whitespace().collect();

    // similar to compressor, turns the input into an iterable based on lines, of which
    // each element is turned into an iterable based on '\t' and then ' '.
    // it's then turned back into words from the dictionary and pasted back together backwards
    // into a string.
    let updated: String = input
        .lines()
        .skip(1)
        .map(|n| {
            n.split_terminator('\t')
                .map(|t| {
                    t.split_terminator(' ')
                        .map(|s| {
                            if !s.is_empty() {
                                let num = s.to_string().parse::<usize>().expect(
                                    "That wasn't a number that we came across to decompress.");
                                if num >= dict.len() {
                                    panic!("It seems that the number exceeds our dictionary size.");
                                }
                                dict[num].to_string()
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

    // writes out and prints statement saying so.
    // let _ = fs::write("files/out_dec.txt", updated);
    println!("File decompressed and can be found here:"); //at 'files/out_dec'.txt");
    println!("{}", updated);
}
