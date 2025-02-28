use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

//no clue how this code works lol
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, 
{
   let file = File::open(filename)?;
   Ok(io::BufReader::new(file).lines())
}

fn calc_distance(lvec: &String, rvec: &String) -> u64
{
	//lvec will ALWAYS be smaller than rvec
	let int_rvec = rvec.parse::<u64>().unwrap();
	let int_lvec = lvec.parse::<u64>().unwrap();
	
	// println!("{} - {}", int_rvec, int_lvec);
	return int_rvec - int_lvec;
}

fn main()
{
	let mut distance: u64 = 0;

	//data for vectors
	let mut lvec: Vec<String> = Vec::new();
	let mut rvec: Vec<String> = Vec::new();

	//fill vectors with input data
	if let Ok(lines) = read_lines("./input")
	{
		for line in lines.map_while(Result::ok)
		{
			let left_value: &str = &line[0..5];
			lvec.push(left_value.to_string());

			let right_value: &str = &line[8..13];
			rvec.push(right_value.to_string());

		}
		
	}
	
	//sort the 2 vectors
	lvec.sort();
	rvec.sort();
	
	for i in 0..lvec.len()
	{
		
		if lvec[i] <= rvec[i]
		{
			// println!("{}, {}", lvec[i], rvec[i]);
			distance = distance + calc_distance(&lvec[i], &rvec[i]);
		}
		else
		{
			
			distance = distance + calc_distance(&rvec[i], &lvec[i]);
		}

	}

	println!("{}", distance);
	
}