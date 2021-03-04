use std::marker::PhantomData;

use cgmath::{Matrix2, Matrix3, Matrix4, Quaternion, Vector2, Vector3, Vector4};

pub struct CacheKey<TType, TKey> {
    pub key: TKey,
    phantom: PhantomData<TType>,
}

pub trait Cache<TKey, TValue> {
    fn allocate(&mut self, val: TValue) -> CacheKey<TValue, TKey>;
    fn get_mut(&mut self, index: &CacheKey<TValue, TKey>) -> Option<&mut TValue>;
    fn get(&self, index: &CacheKey<TValue, TKey>) -> Option<&TValue>;
}

pub trait Update {
    fn order() -> u32;
}

macro_rules! make_props {
    (struct $name:ident {
        type Prop = $prop_val_name:ident;
        type Key = $key:ty;
        iters = [$($iter_type:ty,)*],
        $($field_name:ident: $raw_type:ty,)*
    }) => {
        pub struct $name {
            $($field_name: VecCache<$raw_type>,)*
        }

        pub trait $prop_val_name<T: 'static = Self> {
            fn get_cache_mut(set: &mut $name) -> &mut dyn Cache<$key, T>;
            fn get_cache(set: &$name) -> &dyn Cache<$key, T>;
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                    $($field_name: VecCache::new(),)*
                }
            }

            #[inline]
            pub fn allocate<T: $prop_val_name + 'static>(&mut self, val: T) ->  CacheKey<T, $key> {
                T::get_cache_mut(self).allocate(val)
            }

            #[inline]
            pub fn get_mut<T:$prop_val_name + 'static>(&mut self, index: &CacheKey<T, $key>) -> Option<&mut T> {
                T::get_cache_mut(self).get_mut(index)
            }

            #[inline]
            pub fn get<T: $prop_val_name + 'static>(&self, index: &CacheKey<T, $key>) -> Option<&T> {
                T::get_cache(self).get(index)
            }
        }
        $(
            impl $prop_val_name for $raw_type {
            #[inline]
            fn get_cache_mut(set: &mut $name) -> &mut dyn Cache<$key, $raw_type> {
                    &mut set.$field_name
                }
            #[inline]
            fn get_cache(set: &$name) -> &dyn Cache<$key, $raw_type> {
                    &set.$field_name
                }
            }
        )*
    }
}

make_props! {
    struct PropSet {
        type Prop = PropSetVal;
        type Key = usize;
        iters = [Update,],

        i8_cache: i8,
        vec2_i8_cache: Vector2<i8>,
        vec3_i8_cache: Vector3<i8>,
        vec4_i8_cache: Vector4<i8>,
        mat2_i8_cache: Matrix2<i8>,
        mat3_i8_cache: Matrix3<i8>,
        mat4_i8_cache: Matrix4<i8>,

        u8_cache: u8,
        vec2_u8_cache: Vector2<u8>,
        vec3_u8_cache: Vector3<u8>,
        vec4_u8_cache: Vector4<u8>,
        mat2_u8_cache: Matrix2<u8>,
        mat3_u8_cache: Matrix3<u8>,
        mat4_u8_cache: Matrix4<u8>,

        i16_cache: i16,
        vec2_i16_cache: Vector2<i16>,
        vec3_i16_cache: Vector3<i16>,
        vec4_i16_cache: Vector4<i16>,
        mat2_i16_cache: Matrix2<i16>,
        mat3_i16_cache: Matrix3<i16>,
        mat4_i16_cache: Matrix4<i16>,

        u16_cache: u16,
        vec2_u16_cache: Vector2<u16>,
        vec3_u16_cache: Vector3<u16>,
        vec4_u16_cache: Vector4<u16>,
        mat2_u16_cache: Matrix2<u16>,
        mat3_u16_cache: Matrix3<u16>,
        mat4_u16_cache: Matrix4<u16>,

        i32_cache: i32,
        vec2_i32_cache: Vector2<i32>,
        vec3_i32_cache: Vector3<i32>,
        vec4_i32_cache: Vector4<i32>,
        mat2_i32_cache: Matrix2<i32>,
        mat3_i32_cache: Matrix3<i32>,
        mat4_i32_cache: Matrix4<i32>,

        u32_cache: u32,
        vec2_u32_cache: Vector2<u32>,
        vec3_u32_cache: Vector3<u32>,
        vec4_u32_cache: Vector4<u32>,
        mat2_u32_cache: Matrix2<u32>,
        mat3_u32_cache: Matrix3<u32>,
        mat4_u32_cache: Matrix4<u32>,

        i64_cache: i64,
        vec2_i64_cache: Vector2<i64>,
        vec3_i64_cache: Vector3<i64>,
        vec4_i64_cache: Vector4<i64>,
        mat2_i64_cache: Matrix2<i64>,
        mat3_i64_cache: Matrix3<i64>,
        mat4_i64_cache: Matrix4<i64>,

        u64_cache: u64,
        vec2_u64_cache: Vector2<u64>,
        vec3_u64_cache: Vector3<u64>,
        vec4_u64_cache: Vector4<u64>,
        mat2_u64_cache: Matrix2<u64>,
        mat3_u64_cache: Matrix3<u64>,
        mat4_u64_cache: Matrix4<u64>,

        f32_cache: f32,
        vec2_f32_cache: Vector2<f32>,
        vec3_f32_cache: Vector3<f32>,
        vec4_f32_cache: Vector4<f32>,
        mat2_f32_cache: Matrix2<f32>,
        mat3_f32_cache: Matrix3<f32>,
        mat4_f32_cache: Matrix4<f32>,
        quat_f32_cache: Quaternion<f32>,

        f64_cache: f64,
        vec2_f64_cache: Vector2<f64>,
        vec3_f64_cache: Vector3<f64>,
        vec4_f64_cache: Vector4<f64>,
        mat2_f64_cache: Matrix2<f64>,
        mat3_f64_cache: Matrix3<f64>,
        mat4_f64_cache: Matrix4<f64>,
        quat_f64_cache: Quaternion<f64>,

        bool_cache: bool,
        vec2_bool_cache: Vec2<bool>,
        vec3_bool_cache: Vec3<bool>,
        vec4_bool_cache: Vec4<bool>,

        string_cache: String,
    }
}

pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

struct VecCache<T> {
    values: Vec<T>,
}

impl<T> VecCache<T> {
    fn new() -> VecCache<T> {
        VecCache { values: Vec::new() }
    }
}

impl<T> Cache<usize, T> for VecCache<T> {
    fn allocate(&mut self, val: T) -> CacheKey<T, usize> {
        let index = self.values.len();
        self.values.push(val);
        CacheKey {
            key: index,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn get_mut(&mut self, index: &CacheKey<T, usize>) -> Option<&mut T> {
        self.values.get_mut(index.key)
    }

    #[inline]
    fn get(&self, index: &CacheKey<T, usize>) -> Option<&T> {
        self.values.get(index.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_size() {
        println!("Structure is {} bytes", std::mem::size_of::<PropSet>());
        println!(
            "Cache i32 is {} bytes",
            std::mem::size_of::<VecCache<i32>>()
        );
        println!("Cache i8 is {} bytes", std::mem::size_of::<VecCache<i8>>());
        println!("i32 is {} bytes", std::mem::size_of::<i32>());
        println!("bool is {} bytes", std::mem::size_of::<bool>());
        println!("vec2 bool is {} bytes", std::mem::size_of::<Vec2<bool>>());
    }

    #[test]
    fn bench_add_two_2() {
        let mut set = PropSet::new();
        let key2 = set.allocate::<i32>(123);
        let key4 = set.allocate::<String>(String::from("Tacos"));
        let key = set.allocate::<Vector3<i32>>(Vector3 { x: 1, y: 2, z: 3 });
        *set.get_mut(&key2).unwrap() = 2;
        (
            set.get(&key2).unwrap(),
            set.get(&key2).unwrap(),
            set.get(&key4).unwrap(),
            set.get(&key).unwrap(),
            set.get(&key).unwrap(),
            set.get(&key).unwrap(),
        );
    }
}
