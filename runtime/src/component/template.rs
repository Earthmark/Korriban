pub struct Template {
    defaults: ValueStore,
    fields: Vec<Field>,
}

impl Template {
    fn new() -> Template {
        Template {
            defaults: ValueStore::new(),
            fields: Vec::new(),
        }
    }
}

trait FieldAccessor<Val> {
    fn get(&self, index: usize) -> Option<&Val>;
    fn add(&mut self, value: Val) -> usize;
}

enum ValueType {
    I32 { default: i32 },
    F32 { default: f32 },
}

struct ValueStore {
    ti32: Vec<i32>,
    tf32: Vec<f32>,
}

impl ValueStore {
    fn new() -> ValueStore {
        ValueStore {
            ti32: Vec::new(),
            tf32: Vec::new(),
        }
    }
}

struct Field {
    name: String,
    index: usize,
    value: ValueType,
}
