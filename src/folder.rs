#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Folder {
    pub name: String,
    permissions: Option<String>,
    children: Option<usize>,
    // created: Option<String>,
    // modified: Option<String>,
}

impl Folder {
    #[must_use]
    pub const fn from(name: String, permissions: Option<String>, children: Option<usize>) -> Self {
        Self {
            name,
            permissions,
            children,
        }
    }

    #[must_use]
    pub fn permissions(&self) -> String {
        self.permissions
            .as_deref()
            .map_or("-".repeat(10), ToString::to_string)
    }

    #[must_use]
    pub fn children(&self) -> usize {
        self.children.map_or(0, |num| num)
    }
}
