
#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub extension: String,
}

impl Package {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            architecture: "x86_64".into(),
            extension: ".bloxpkg".into(),
        }
    }

    pub fn validate(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.version.trim().is_empty()
            && self.extension == ".bloxpkg"
    }
}

fn main() {
    let package = Package::new("BloxOS-System", "1.0.0");
    println!("{package:?}");
}
