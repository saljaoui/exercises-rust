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
        } else if self.password.len() < 8 {
        return Err( FormError::new("password", self.name.clone(), "Password should be at least 8 characters long") )
        }
        Ok(())
    }
}