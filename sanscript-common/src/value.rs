pub type Number = f64;

#[derive(Clone, PartialEq)]
pub enum Value {
    ValBool(bool),
    ValNumber(Number),
    ValNil,
    ValString(String)
}

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
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

    pub fn print_value(value: &Value) {
        match value {
            Value::ValBool(boolean) => print!("\x1B[3m{}\x1B[0m", boolean),
            Value::ValNumber(number) => print!("\x1B[3m{}\x1B[0m", number),
            Value::ValNil => print!("\x1B[3m{}\x1B[0m", "nil"),
            Value::ValString(string) => print!("\x1B[3m{}\x1B[0m", string)
        }
    }
}
