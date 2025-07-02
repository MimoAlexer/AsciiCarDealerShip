pub struct CarFrame {
    string: String,
}

impl CarFrame {
    pub fn new() -> Self {
        CarFrame {
            string: String::from(
                "o                       o |=======================| |  []   []   []   []    | |=======================|  o                       o"
            )
        }
    }

    pub fn get_string(&self) -> &str {
        &self.string
    }
}