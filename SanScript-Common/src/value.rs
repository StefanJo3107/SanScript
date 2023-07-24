pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>
}

impl ValueArray{
    pub fn new() -> ValueArray {
        ValueArray { values: vec![] }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, index: usize) -> &Value {
        &self.values[index]
    }

    pub fn write_constant(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn print_value(value: &Value){
        print!("\x1B[3m{}\x1B[0m", value);
    }
}
