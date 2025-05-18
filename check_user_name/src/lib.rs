#[derive(Debug, PartialEq)]
pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    acessLevel: AccessLevel
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
    User {
        name: name,
        acessLevel: level,
    }
  }
  pub fn send_name(&self) -> Option<&str> {
    println!("{:?}", self);
     if self.acessLevel == AccessLevel::Guest {
        return None;
    } else {
        return Some(&self.name);
    }
  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    if user.acessLevel == AccessLevel::Guest {
        return (false, "ERROR: User is guest");
    } else {
        return (true, &user.name);
    }
}

