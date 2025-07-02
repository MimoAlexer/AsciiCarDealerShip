pub struct CarFrame {
    string: String,
}

impl CarFrame {
    pub fn new() -> Self {
        CarFrame {
            string: String::from(
                "o                       o
                    |=======================|
                    |  []   []   []   []    |
                    |=======================|
                    o                       o"
            )
        }
    }
}