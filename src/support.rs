pub struct Folder {
    name: String,
    size: Option<u32>,
}

pub struct File {
    name: String,
    size: Option<u32>,
}

impl Folder {
    pub fn from(name: String, size: Option<u32>) -> Self {
        Self { name, size }
    }
}

impl File {
    pub fn from(name: String, size: Option<u32>) -> Self {
        Self { name, size }
    }
}
