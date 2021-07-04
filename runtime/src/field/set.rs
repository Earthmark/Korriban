use super::types::{FieldType, FieldTypeBind};

struct Set {
    vi32: Vec<i32>,
}

trait Provider<T: FieldTypeBind> {
    fn get(&self) -> T;
}

impl Provider<i32> for Set {
    fn get(&self) -> i32 {
        0
    }
}

impl Set {
    fn get<T: FieldTypeBind + Default>(&self) -> T {
        T::default()
    }
}
