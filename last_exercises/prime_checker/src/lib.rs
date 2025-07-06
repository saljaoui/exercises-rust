#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {

    if nb < 2 {
        return None;
    }

    if nb == 2 {
        return Some(Ok(nb));
    }

    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }

    for i in 2..1000000 {
        if nb % i == 0  && nb != i {
            return Some(Err(PrimeErr::Divider(i)));
        }
    }

    Some(Ok(nb))
}
