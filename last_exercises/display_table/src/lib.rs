#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}
use std::fmt;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// println!("{}", self);
		if self.headers.len() == 0 {
			return write!(f, "");
		}
	let mut vec: Vec<usize> = vec![];
	for header in &self.headers {
		// println!("{:?}", header.clone());
		vec.push(header.len());
	}

	for b in &self.body {
		for i in 0..b.len() {
			if b[i].len() > vec[i] {
				vec[i] = b[i].len()
			}
		}
		
	}

	let mut res = String::new();

	// for header in &self.headers {
	// 	println!("{:?}", header.clone());
	// 	let n_spaces = 
	// 	// vec.push(header.len());
	// }
	res.push_str(&fix_words(&self.headers, &vec));
	
	res.push_str(&create_separator_line(&vec));
	
	for b in & self.body {
		res.push_str(&fix_words(&b, &vec))
	}
	// println!("{:?}", res);

		write!(f, "{}", res)
    }
}

fn create_separator_line(vec_len: &Vec<usize>) -> String {
	let mut res = String::new();
	res.push('|');
	for i in 0..vec_len.len() {
		
		for _ in 0..(vec_len[i] + 2) {
			res.push('-');
		}
		if i < vec_len.len() - 1 {
			res.push('+');
		}
		
	}
	res.push('|');
	res.push('\n');
	
	res
}

fn for_space(n: usize) -> String {
	let mut i = 0;
	let mut res = String::new();
	while i < n {
		res.push(' ');
		i +=1
	}
	res
}

fn fix_words(vec: &Vec<String>, vec_len: &Vec<usize>) -> String {
	let mut res = String::new();
	let mut add_space = false;
	// println!("{:?}", vec);
	for i in 0..vec.len() {
		// println!("{:?}", vec[i]);
		let mut n_spaces = vec_len[i] - vec[i].chars().count();
		if n_spaces % 2 != 0 {
			add_space = true;
			n_spaces = n_spaces / 2;
		} else {
			n_spaces = n_spaces / 2;
		}
		res.push('|');
		res.push_str(&for_space(n_spaces+1));
		res.push_str(&vec[i]);
		res.push_str(&for_space(n_spaces+1));


		if add_space {
			add_space = false;
			res.push_str(&for_space(1));
		}
	}
	res.push('|');
	res.push('\n');

	// println!("{:?}", res);
	res
}

impl Table {
	pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new()
        }
	}

	pub fn add_row(&mut self, row: &[String]) {
		self.body.push(row.into())
	}
}