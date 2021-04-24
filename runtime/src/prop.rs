use std::{marker::PhantomData, usize};

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

        i32_cache: i32,
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
    }

    #[test]
    fn bench_add_two_2() {
        let mut set = PropSet::new();
        let key2 = set.allocate::<i32>(123);
        *set.get_mut(&key2).unwrap() = 2;
        (
            set.get(&key2).unwrap(),
            set.get(&key2).unwrap(),
        );
    }
}
