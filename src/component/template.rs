use glam::{
    Vec2, Vec3, Vec4,
    DVec2, DVec3, DVec4,
    IVec2, IVec3, IVec4,
    UVec2, UVec3, UVec4,
    Mat2, Mat3, Mat4,
    DMat2, DMat3, DMat4,
};

struct Template {
    defaults: ValueStore,
    root: Node,
}

impl Template {
    fn new() -> Template {
        Template {
            defaults: ValueStore::new(),
            root: Node::Object {
                fields: Vec::new(),
            },
        }
    }
}

trait FieldAccessor<Val> {
    fn get(&self, index: usize) -> Option<&Val>;
    fn add(&mut self, value: Val) -> usize;
}

macro_rules! value_space {
    (struct $name:ident => $enum:ident {
        $($nam:ident : $typ:ty,)*
    }) => {
        enum $enum {
            $($nam,)*
        }

        struct $name {
            $($nam: Vec<$typ>,)*
        }

        impl $name {
            fn new() -> $name {
                $name {
                    $($nam: Vec::new(),)*
                }
            }
        }

        $(
            impl FieldAccessor<$typ> for $name {
                fn get(&self, index: usize) -> Option<&$typ> {
                    self.$nam.get(index)
                }

                fn add(&mut self, value: $typ) -> usize {
                    let size = self.$nam.len();
                    self.$nam.push(value);
                    size
                }
            }
        )*
    };
}

value_space!{
    struct ValueStore => ValueType {
        ni32: i32,
        nu32: u32,
        ni64: i64,
        nu64: u64,
        nf32: f32,
        nf64: f64,
        vec2: Vec2,
        vec3: Vec3,
        vec4: Vec4,
        dvec2: DVec2,
        dvec3: DVec3,
        dvec4: DVec4,
        ivec2: IVec2,
        ivec3: IVec3,
        ivec4: IVec4,
        uvec2: UVec2,
        uvec3: UVec3,
        uvec4: UVec4,
        mat2: Mat2,
        mat3: Mat3,
        mat4: Mat4,
        dmat2: DMat2,
        dmat3: DMat3,
        dmat4: DMat4,
    }
}

struct Field {
    name: String,
    index: usize,
    value: Node,
}

enum Node {
    Object {
        fields: Vec<Field>,
    },
    Value {
        typ: ValueType,
        index: usize,
    },
    Array {
        values: Vec<Node>,
    }
}
