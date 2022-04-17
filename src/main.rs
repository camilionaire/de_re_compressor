use std::{env, fs};

fn main() {
//    println!("Hello, world!");
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() != 2 {
		println!("to run:\ncargo run <compress/decompress> <filename>");
		std::process::exit(1);
	}

	let contents = fs::read_to_string(args[1].clone()).expect("something went wrong with the file name...");

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
	println!("this is the compressor test");
	println!("this is the input:\n{}", input);
}

fn decompressor(input: String) {
	println!("this is the decompressor test");
	println!("this is the input:\n{}", input);

	let dict: Vec<&str> = input.lines().next().unwrap().split_whitespace().collect();

	let body = input.lines().skip(1).collect::<Vec<&str>>().join("\n");
	
	let updated: String = body.split_terminator('\n')
								.map(|n| {
									n.split_terminator('\t')
									.map(|t| {
										t.split_terminator(' ')
										.map(|s| {
											// println!("THIS IS S!:{:?}", s);
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
								.collect::<Vec<String>>().join("\n");
	println!("This is our dict:\n{:?}", dict);
	println!("This is our body:\n{}", body);
	println!("This is our updated:\n{}", updated);

	let _ = fs::write("out_dec.txt", updated);


	// original go at the changing function, but didn't account for anything 
	// past the 0-9 digits.
	// let updated: String = body.chars().map(|c| {
	// 	if c.is_numeric() {
	// 		let n: usize = c.to_string().parse().unwrap();
	// 		dict[n].to_string()
	// 	} else {
	// 		c.to_string()
	// 	}
	// }).collect();
}








	// NOTE this is the lame stuff that I wrote for the thing.
	// let mut uniq: Vec<&str> = contents.split_whitespace().collect::<HashSet<_>>().into_iter().collect();
	// let _ = uniq.sort_unstable();
	// println!("{:?}", uniq);

	// let compressed = contents.chars();

	// println!("{:?}", compressed);

	// fs::write("./output.txt", contents.in)

	// let mut phrase: String = "".to_string();
	// for line in contents.lines() {

	// }

	// let lines = contents.lines();

	// for line in lines {
	// 	let words: Vec<&str> = line.split_whitespace().collect();
	// }
	// println!("The text is:\n{}", contents);
