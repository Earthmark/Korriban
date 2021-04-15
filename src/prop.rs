use std::{marker::PhantomData, usize};

use glam::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4, Quat, DVec2, DVec3, DVec4, DMat2, DMat3, DMat4, DQuat, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, BVec2, BVec3, BVec4};

type Key = usize;

#[macro_export]
macro_rules! function_component {
    ($prop:expr, [$($access:expr),*], $func:block, ) => {
        ($(*$write.get_mut($prop),)*) = $func($($access.get($prop),)*)
    };
}

pub struct CacheKey<TType> {
    pub key: Key,
    phantom: PhantomData<TType>,
}

impl<TType> CacheKey<TType> {
    pub fn new(key: Key) -> CacheKey<TType> {
        CacheKey {
            key,
            phantom: PhantomData,
        }
    }
}

pub trait Cache<TValue> {
    fn allocate(&mut self, val: TValue) -> CacheKey<TValue>;
    fn get_mut(&mut self, index: &CacheKey<TValue>) -> Option<&mut TValue>;
    fn get(&self, index: &CacheKey<TValue>) -> Option<&TValue>;
}

pub trait Update {
    fn order(&self) -> u32;
    fn update(&self, props: &mut PropSet);
}

macro_rules! make_props {
    (struct $name:ident {
        type Prop = $prop_val_name:ident;
        iters = [$($iter_type:ty,)*],
        $($field_name:ident: $raw_type:ty,)*
    }) => {
        pub struct $name {
            $($field_name: VecCache<$raw_type>,)*
        }

        pub trait $prop_val_name<T: 'static = Self> {
            fn get_cache_mut(set: &mut $name) -> &mut dyn Cache<T>;
            fn get_cache(set: &$name) -> &dyn Cache<T>;
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                    $($field_name: VecCache::new(),)*
                }
            }

            #[inline]
            pub fn allocate<T: $prop_val_name + 'static>(&mut self, val: T) ->  CacheKey<T> {
                T::get_cache_mut(self).allocate(val)
            }

            #[inline]
            pub fn get_mut<T:$prop_val_name + 'static>(&mut self, index: &CacheKey<T>) -> Option<&mut T> {
                T::get_cache_mut(self).get_mut(index)
            }

            #[inline]
            pub fn get<T: $prop_val_name + 'static>(&self, index: &CacheKey<T>) -> Option<&T> {
                T::get_cache(self).get(index)
            }
        }
        $(
            impl $prop_val_name for $raw_type {
            #[inline]
            fn get_cache_mut(set: &mut $name) -> &mut dyn Cache<$raw_type> {
                    &mut set.$field_name
                }
            #[inline]
            fn get_cache(set: &$name) -> &dyn Cache<$raw_type> {
                    &set.$field_name
                }
            }
        )*
    }
}

make_props! {
    struct PropSet {
        type Prop = PropSetVal;
        iters = [Update,],

        i32_cache: i32,
        vec2_i32_cache: IVec2,
        vec3_i32_cache: IVec3,
        vec4_i32_cache: IVec4,

        u32_cache: u32,
        vec2_u32_cache: UVec2,
        vec3_u32_cache: UVec3,
        vec4_u32_cache: UVec4,

        f32_cache: f32,
        vec2_f32_cache: Vec2,
        vec3_f32_cache: Vec3,
        vec4_f32_cache: Vec4,
        mat2_f32_cache: Mat2,
        mat3_f32_cache: Mat3,
        mat4_f32_cache: Mat4,
        quat_f32_cache: Quat,

        f64_cache: f64,
        vec2_f64_cache: DVec2,
        vec3_f64_cache: DVec3,
        vec4_f64_cache: DVec4,
        mat2_f64_cache: DMat2,
        mat3_f64_cache: DMat3,
        mat4_f64_cache: DMat4,
        quat_f64_cache: DQuat,

        bool_cache: bool,
        vec2_bool_cache: BVec2,
        vec3_bool_cache: BVec3,
        vec4_bool_cache: BVec4,

        string_cache: String,
    }
}

struct VecCache<T> {
    values: Vec<T>,
}

impl<T> VecCache<T> {
    fn new() -> VecCache<T> {
        VecCache { values: Vec::new() }
    }
}

impl<T> Cache<T> for VecCache<T> {
    fn allocate(&mut self, val: T) -> CacheKey<T> {
        let index = self.values.len();
        self.values.push(val);
        CacheKey {
            key: index,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn get_mut(&mut self, index: &CacheKey<T>) -> Option<&mut T> {
        self.values.get_mut(index.key)
    }

    #[inline]
    fn get(&self, index: &CacheKey<T>) -> Option<&T> {
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
        println!("vec2 bool is {} bytes", std::mem::size_of::<BVec2>());
    }

    #[test]
    fn bench_add_two_2() {
        let mut set = PropSet::new();
        let key2 = set.allocate::<i32>(123);
        let key4 = set.allocate::<String>(String::from("Tacos"));
        let key = set.allocate::<IVec3>(IVec3::new(1, 2,3));
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
