pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray {
            values: vec![]
        }
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn print_value(&self, idx: usize) {
        print!("{}", self.values[idx]);
    }
}
