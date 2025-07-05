use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>, pub u32);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        // println!("{:?}", n);
    match n {
        1 => RomanDigit::I,
        5 => RomanDigit::V,
        10 => RomanDigit::X,
        50 => RomanDigit::L,
        100 => RomanDigit::C,
        500 => RomanDigit::D,
        1000 => RomanDigit::M,
        _ => RomanDigit::Nulla,
    }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let mut vec: Vec<RomanDigit> = vec![];
        // println!("{:?}", n);
        let mut s = n;
        let befor = 0;
        while s > 0 {
            if s >= 1000 {
            vec.push(RomanDigit::from(1000));
            s -= 1000;
            } else if s >= 900 {
            vec.push(RomanDigit::from(100));
            vec.push(RomanDigit::from(1000));
            s -= 900;
            } else if s >= 500 {
            vec.push(RomanDigit::from(500));
            s -= 500;
            } else if s >= 400 {
            vec.push(RomanDigit::from(100));
            vec.push(RomanDigit::from(500));
            s -= 400;
            } else if s >= 100 {
            vec.push(RomanDigit::from(100));
            s -= 100;
            } else if s >= 90 {
                vec.push(RomanDigit::from(10));
            vec.push(RomanDigit::from(100));
            s -= 90;
            } else if s >= 50 {
            vec.push(RomanDigit::from(50));
            s -= 50;
        } else  if s >= 40 {
            vec.push(RomanDigit::from(10));
            vec.push(RomanDigit::from(50));
            s -= 40;
        } else if s >= 10 {
            vec.push(RomanDigit::from(10));
            s -= 10;
        } else if s >= 9 {
            vec.push(RomanDigit::from(1));
            vec.push(RomanDigit::from(10));
            s -= 9;
        }  else if s >= 5 {
            vec.push(RomanDigit::from(5));
            s -= 5;
        } else if s >= 4 {
            vec.push(RomanDigit::from(1));
            vec.push(RomanDigit::from(5));
            s -= 4;
        } else {
            vec.push(RomanDigit::from(1));
            s -= 1;
        }
        } 
        // if s <= 50 {
        //     let f = s % 50;
        //     println!(">{:?}", RomanDigit::from(s));
        // }


        Self (
            vec,
            n
        )
    }
}











impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
        Some(RomanNumber::from(self.1+1))
    }
}
