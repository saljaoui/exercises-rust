#[derive(PartialEq)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

use crate::Security::*;

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    if security_level == Warning {
        return server.unwrap_or("WARNING: check the server").to_string();
    } else if security_level == NotFound {
        if let Ok(data) = server {
            return data.to_string();
        } else if let Err(data) = server {
            return format!("Not found: {}", data);
        }
    } else if security_level == Unknown {
        if let Ok(data) = server {
            panic!("{:?}", data);
        } else if let Err(data) = server {
            panic!("{:?}", data);
        }
    } else if security_level == Message {
        if let Ok(data) = server {
            return data.to_string();
        } else {
            panic!("ERROR: program stops");
        }
    } else if security_level == UnexpectedUrl {
        if let Err(data) = server {
            return data.to_string();
        } else if let Ok(data) = server {
            panic!("{}", data);
        }
    }
    todo!()
}