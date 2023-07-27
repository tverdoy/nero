pub struct IntArgs {
    pub max: Option<i32>,
    pub min: Option<i32>,
}

impl IntArgs {
    pub const fn default() -> Self {
        Self {
            max: None,
            min: None,
        }
    }
}

pub struct StringArg {
    pub max_len: Option<usize>,
}

impl StringArg {
    pub const fn default() -> Self {
        Self { max_len: None }
    }
}
