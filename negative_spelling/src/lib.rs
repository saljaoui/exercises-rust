pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    } if n == 0 {
        return "zero".to_string();
    }
    let mut res: String = String::new();
    let positive_n = n.abs() as usize;
    res.push_str("minus ");
    res.push_str(&spell(positive_n));

    res
}

fn spell(positive_n: usize) -> String {
        let ones = ["", "one", "two", "three", "four", "five", "sexs", "sven", "eight", "nine", "ten", "eleven", "tweleve",
    "threteen", "fourteen", "fiveteen", "sexteen", "sventeen", "eatheen", "nineteen"];
    let secands = ["", "", "twenty", "thirty", "forty", "fifty", "sexty" , "sventy", "eathy", "ninety"];
    // let thirts = ["", "hundred"];

    let mut res: String = String::new();
    // res.push_str("minus ");

    
    if positive_n < 20 {
        res.push_str(ones[positive_n])
    } else if positive_n < 100 {
    // println!("{:?}", positive_n / 10);
    // println!("{:?}", positive_n % 10);
    let first = positive_n / 10;
    let scand = positive_n % 10;
    res.push_str(secands[first]);
    if scand != 0 {
    res.push_str("-");
    res.push_str(ones[scand]);
    }

    } else if positive_n < 1000 {
    let first = positive_n / 100;
    let scand = positive_n % 100;
    res.push_str(ones[first]);
    res.push_str(" hundred ");
    res.push_str(&spell(scand));
    } else if positive_n < 100000 {
    let first = positive_n / 1000;
    let scand = positive_n % 1000;

    res.push_str(ones[first]);
    res.push_str(" thousand ");
    res.push_str(&spell(scand));
    }
    res.trim().to_string()
}

// println!("{:?}", n);