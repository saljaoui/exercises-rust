fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}

fn brain_fuck(arg: &str) {

    let mut vec: Vec<i32> = vec![0; 10];
    let mut point = 0;

    for a in arg.chars() {

        if a == '+' {
            vec[point] += 1;
        } else if a == '-' {
            vec[point] -= 1;
        } else if a == '>' {
            point += 1;
        } else if a == '<' {
            if point > 0 {
                point -= 1;
            }
        } else if a == ']' {
            for i in 0..vec.len() {
                vec[i] *= 10
            }
        } else if a == '.' {
            print!("{}", vec[point] as u8 as char);
        }
    }

    // println!("{:?}", vec); // optional debug
}











#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../brain_fuck/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn nothing_passed() {
        assert_eq!("", run(""));
    }

    #[test]
    fn single_chars() {
        assert_eq!(
            "a",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.")
        );
        assert_eq!(
            "S",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++++++++.")
        );
        assert_eq!(
            "7",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>---------------.")
        );
    }
    #[test]
    fn phrases() {
        assert_eq!(
            "Wow",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>-------------.++++++++++++++++++++++++.++++++++.")
        );
        assert_eq!(
            "Good job!",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+.>+++++++++++..-----------.<<++.>>++++++.+++++.-------------.<<+.")
        );
    }

    #[test]
    fn with_characters_in_middle() {
        assert_eq!("keep going", run("++++++++++[>+>ke+++>+++++++>++++++++++<<<<-]>>>>+++++++e.------p..+++++++++++.<<++.>g>---------.+o+++++++.------i.+++++.-n------.g"));
    }

    #[test]
    fn big_test() {
        assert_eq!(
            "3, 2, 1... Happy New Year",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>-------------------.-------.<++.>++++++.------.<.>+++++.---...<.>++++++++++++++++++++++++++.>---.+++++++++++++++..+++++++++.<<.>++++++.>--------------------.++++++++++++++++++.<<.>+++++++++++.++++++++++++.----.>-----.")
        );
        assert_eq!(
            "To be or not be, that is the question!", 
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++++++++++++++.>+++++++++++.<<++.>>-------------.+++.<<.>>++++++++++.+++.<<.>>----.+.+++++.<<.>++++++++++++++.+++.<++++++++++++.------------.>>.<+++.-------.>.<<.>++++++++.>-.<<.>>+.<-.---.<.>>---.++++.<.>--.+.<++++.>-----.-.<<+.")
        );
    }
}