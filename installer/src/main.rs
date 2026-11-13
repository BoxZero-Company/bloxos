
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStep {
    Welcome,
    Language,
    Account,
    Purpose,
    Network,
    Partition,
    Summary,
    Installing,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPurpose {
    Personal,
    Development,
    Education,
    Server,
}

pub struct Installer {
    pub step: InstallStep,
    pub language: &'static str,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub recovery: String,
    pub purpose: Option<InstallPurpose>,
}

impl Installer {
    pub fn new() -> Self {
        Self {
            step: InstallStep::Welcome,
            language: "English",
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            recovery: String::new(),
            purpose: None,
        }
    }

    pub fn validate_account(&self) -> Result<(), &'static str> {
        if self.username.trim().is_empty() {
            return Err("Username is required");
        }

        if self.password.is_empty() {
            return Err("Password is required");
        }

        if self.confirm_password.is_empty() {
            return Err("Password confirmation is required");
        }

        if self.recovery.trim().is_empty() {
            return Err("Recovery information is required");
        }

        if self.password != self.confirm_password {
            return Err("Passwords do not match");
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), &'static str> {
        let next = match self.step {
            InstallStep::Welcome => InstallStep::Language,

            InstallStep::Language => InstallStep::Account,

            InstallStep::Account => {
                self.validate_account()?;
                InstallStep::Purpose
            }

            InstallStep::Purpose => {
                if self.purpose.is_none() {
                    return Err("Select an installation purpose");
                }

                InstallStep::Network
            }

            InstallStep::Network => InstallStep::Partition,

            InstallStep::Partition => InstallStep::Summary,

            InstallStep::Summary => InstallStep::Installing,

            InstallStep::Installing => InstallStep::Complete,

            InstallStep::Complete => InstallStep::Complete,
        };

        self.step = next;
        Ok(())
    }
}

fn main() {
    let installer = Installer::new();

    println!("BloxOS Installer");
    println!("Version: 0.1.0");
    println!("Current step: {:?}", installer.step);
}
