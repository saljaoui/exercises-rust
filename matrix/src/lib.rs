#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar {
	type Item;
	fn zero() -> Self::Item;
    fn identity() -> Self::Item;
	// fn one() -> Self::Item;
}

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![]])
	}

	// pub fn zero(row: usize, col: usize) -> Matrix<T> {
	// }

	pub fn identity(n: usize) -> Matrix<T> {
        println!("{:?}", n);
        Matrix(vec![vec![]])
	}
}


