#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct File {
    pub name: String,
    size: Option<u64>,
    permissions: Option<String>,
    // created: Option<String>,
    // modified: Option<String>,
}

impl File {
    #[must_use]
    pub const fn from(name: String, size: Option<u64>, permissions: Option<String>) -> Self {
        Self {
            name,
            size,
            permissions,
        }
    }

    #[must_use]
    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .map_or(String::new(), ToString::to_string)
    }

    #[must_use]
    pub fn size(&self) -> u64 {
        self.size.map_or(0, |num| num)
    }
}
