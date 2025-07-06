#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
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

	pub fn filter_col(&self, filter: impl Fn(&str) -> bool) -> Option<Self> {
		let mut table = Table::new();
		
		for i in 0..self.headers.len() {
			if filter(&self.headers[i].clone()) {
					table.headers.push(self.headers[i].clone());
					for s in 0..self.body.len() {
						table.body.push(vec![self.body[s][i].clone()]);
					}
			}
		}
		Some(table)
	}

	pub fn filter_row(&self, col_name: &str, filter: impl Fn(&str) -> bool) -> Option<Self> {
		let mut table = Table::new();
		// println!("{:?}", col_name);
		// println!("{:?}", self);
		table.headers = self.headers.clone();
		for i in 0..self.headers.len() {
			if &self.headers[i].clone() == col_name {
				// println!("{:?}", self.body[i]);
				
				for s in 0..self.body.len() {
					if filter(&self.body[s][i].clone()) {
						table.body.push(self.body[s].clone());
					}
				}
			}
		}
		Some(table)
	}
}