#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum FieldType {
    Bool,
    I32,
    F32,
    V3F32,
}

pub enum SepcialType {}

pub trait FieldTypeBind {
    fn field_type() -> FieldType;
}

impl FieldTypeBind for i32 {
    fn field_type() -> FieldType {
        FieldType::I32
    }
}

impl FieldTypeBind for f32 {
    fn field_type() -> FieldType {
        FieldType::F32
    }
}

impl FieldTypeBind for bool {
    fn field_type() -> FieldType {
        FieldType::Bool
    }
}
