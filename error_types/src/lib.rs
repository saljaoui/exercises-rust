use chrono::Local;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FormError {
    form_values: (String, String),
    date: String,
    err: String,

}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values: (field_name.to_string(), field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.password.len() < 8 {
            return Err( FormError::new("password", self.password.clone(), "Password should be at least 8 characters long") )
        }

        let mut is_ascii_digit: bool = false;
        let mut is_ascii_alphabetic: bool = false;
        let mut is_ascii_punctuation: bool = false;

        for p in self.password.chars() {
            if p.is_ascii_digit() {
                is_ascii_digit = true;
            } else if p.is_ascii_alphabetic() {
                is_ascii_alphabetic = true;
            } else if p.is_ascii_punctuation() {
                is_ascii_punctuation = true;
            }
        }

        if !is_ascii_digit || !is_ascii_alphabetic || !is_ascii_punctuation {
            return Err( FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols") )
        }

        if self.name.is_empty() {
        return Err( FormError::new("name",  self.name.clone(), "Username is empty") )
        }
        if self.password.len() < 8 {
        return Err( FormError::new("password", self.name.clone(), "Password should be at least 8 characters long") )
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type() {
        let cases = [
            (
                Form {
                    name: "Katy".to_owned(),
                    password: "qwTw12&%$3sa1dty_".to_owned(),
                },
                Ok(()),
            ),
            (
                Form {
                    name: "".to_owned(),
                    password: "qwTw12&%$3sa1dty_".to_owned(),
                },
                Err(("name".to_string(), "".to_owned(), "Username is empty".to_string())),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "12345".to_owned(),
                },
                Err(("password".to_string(), "12345".to_owned(), "Password should be at least 8 characters long".to_string())),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "sdASDsrW".to_owned(),
                },
                Err(("password".to_string(), "sdASDsrW".to_owned(), "Password should be a combination of ASCII numbers, letters and symbols".to_string())),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "dsGE1SAD213".to_owned(),
                },
                Err(("password".to_string(), "dsGE1SAD213".to_owned(), "Password should be a combination of ASCII numbers, letters and symbols".to_string())),
            ),
            (
                Form {
                    name: "Someone".to_owned(),
                    password: "dsaSD&%DF!?=".to_owned(),
                },
                Err(("password".to_string(), "dsaSD&%DF!?=".to_owned(), "Password should be a combination of ASCII numbers, letters and symbols".to_string())),
            ),
        ];

        for (form, expected) in cases {
            match (form.validate(), expected) {
                (Ok(()), Ok(())) => {} // all good
                (Err(actual), Err((field, value, msg))) => {
                    assert_eq!(actual.form_values, (field, value));
                    assert_eq!(actual.err, msg);
                }
                (result, expected) => {
                    panic!("Mismatch!\nGot: {:?}\nExpected: {:?}", result, expected);
                }
            }
        }
    }
}
