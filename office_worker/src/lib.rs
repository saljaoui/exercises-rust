// use crate::OfficeWorker::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name : String,
    pub age : u32,
    pub role : WorkerRole
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin, 
    User,
    Guest
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let split: Vec<_> = s.split(",").collect();
        OfficeWorker {
                name : split[0].to_string(),
    age : split[1].parse::<u32>().expect("REASON"),
    role : WorkerRole::from(split[2])
        }
    }
}

impl From<&str> for WorkerRole {
     fn from(s: &str) -> Self {
        if s == "admin" {
            return WorkerRole::Admin;
        } else if s == "user" {
            return WorkerRole::User;

        }
        WorkerRole::Guest
     }

}