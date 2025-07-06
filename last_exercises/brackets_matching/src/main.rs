

fn main() {
    let args: Vec<String> = std::env::args().collect();


    for index in 1..args.len() {

    
    let arg: String = args[index].clone();
    let mut vec: Vec<char> = vec![];

    for a in arg.chars() {
       if a == '(' || a == '{' || a == '[' || a == ')' || a == '}' || a == ']' {
        vec.push(a)
       } 
    }

    // println!("{:?}", vec);
    // let mut is_ok = false;
    let mut i = 0;
    while i < vec.len() {
        if i < vec.len() - 1 && ( vec[i] == '(' || vec[i] == '{' || vec[i] == '[') {
            if i < vec.len() && (vec[i+1] == ')' || vec[i+1] == '}' || vec[i+1] == ']') {
                if  vec[i] == '(' && vec[i+1] == ')' {
                vec.remove(i);
                vec.remove(i);
                i = 0
                } else if vec[i] == '{' && vec[i+1] == '}' {
                vec.remove(i);
                vec.remove(i);
                i = 0
                } else if vec[i] == '[' && vec[i+1] == ']' {
                vec.remove(i);
                vec.remove(i);
                i = 0
                } else {
                    break
                }
                
            } else if i < vec.len() && (vec[i+1] != '(' || vec[i+1] != '{' || vec[i+1] != '[') {
                i += 1
            } 
        } else {break}
        // i+=1
    }
    // vec.remove(0);
    // println!("{:?}", vec);

     if vec.is_empty() {
            println!("OK");
    } else {
            println!("Error");
    }
    }

}











#[cfg(test)]
mod tests {

    use rand::distr::Alphanumeric;
    use rand::{thread_rng, Rng};

    use std::process::{Command, Output};

    const MANIFEST_PATH: &str = "../solution/brackets_matching/Cargo.toml";

    fn run(s: Vec<&str>) -> Output {
        Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .args(s.iter())
            .output()
            .expect("Failed to execute command")
    }

    #[test]
    fn random_tests() {
        fn random_alnum() -> String {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect()
        }
        let mut args = vec![
            String::from("(johndoe)"),
            String::from("()"),
            String::from("([])"),
            String::from("{2*[d - 3]/(12)}"),
        ];

        for _ in 0..3 {
            args.push(format!("({:?})", &random_alnum()));
            args.push(format!("[{:?}]", &random_alnum()));
            args.push(format!("{}{:?}{}", "{", &random_alnum(), "}"));
        }

        for v in args.iter() {
            let output = run(vec![v]);
            assert_eq!(String::from_utf8(output.stdout).unwrap(), "OK\n");
        }
    }

    #[test]
    fn tests_both() {
        struct Test<'a> {
            arguments: ([&'a str; 2], &'a str),
        }

        let arr: [Test; 3] = [
            Test {
                arguments: (["", "{[(0 + 0)(1 + 1)](3*(-1)){()}}"], "OK\nOK\n"),
            },
            Test {
                arguments: (["{][]}", "{3*[21/(12+ 23)]}"], "Error\nOK\n"),
            },
            Test {
                arguments: (["{([)])}", "{{{something }- [something]}}"], "Error\nOK\n"),
            },
        ];

        for t in arr.iter() {
            let output = run(t.arguments.0.to_vec());
            assert_eq!(String::from_utf8_lossy(&output.stdout), t.arguments.1);
        }
    }

    #[test]
    fn tests_with_nothing() {
        let output = run(vec![]);
        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    }
}


























