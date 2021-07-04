use super::types::{FieldType, FieldTypeBind};
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub trait Storage<T> {
    fn get(&self, i: usize) -> Option<&T>;
    fn set(&mut self, i: usize, value: T);
}

// This needs to go backwards, instead of binding from source to destination, bind from destination to source.
// This is more viable because it solves the multi-drive problem by having a component deal with that directly.
// Now, each binding does not require a precidence.
// This may not be compatible with the neos style.

// phases
// 1: Exist to get iterator
// 2: queue updates asynchronously
// 3: Process updates in one pulse (flash copy at start).

// unknown requirements: Still need to figure out how to describe multi-rentrant sections of code,
// unsure how to describe those patterns and restart those subgraphs.

pub struct ElementSequencer {
    vals: BTreeMap<FieldType, Vec<usize>>,
}

impl ElementSequencer {
    pub fn update(&mut self) {
        todo!();
    }

    pub fn create_element(&mut self) -> Element {
        Element{
            id: 0,
        }
    }

    fn get_index(&self, key: &FieldKey) -> Option<usize> {
        if let Some(redirect) = self.vals.get(&key.t) {
            if let Some(index) = redirect.get(key.i) {
                return Some(*index);
            }
        }
        return None;
    }
}

pub struct Element {
    id: usize,
}

impl Element {
    pub fn create_producer<T: FieldTypeBind + Default + Eq>(&self, seq: &mut ElementSequencer) -> Producer<T> {
        Producer {
            i: 0,
            p: PhantomData,
        }
    }

    pub fn create_consumer<T: FieldTypeBind + Default + Eq>(&self, seq: &mut ElementSequencer, flags: ConsumerFlags) -> Consumer<T> {
        Consumer {
            i: 0,
            p: PhantomData,
            f: flags,
        }
    }

    pub fn create_bool_trigger_consumer(&self, seq: &mut ElementSequencer, flags: ConsumerFlags) -> Consumer<bool> {
        Consumer {
            i: 0,
            p: PhantomData,
            f: flags,
        }
    }
}

struct FieldKey {
    t: FieldType,
    i: usize,
}

impl FieldKey {
    fn new<T: FieldTypeBind>(i: usize) -> FieldKey {
        FieldKey {
            i,
            t: T::field_type(),
        }
    }
}
pub struct Producer<T: FieldTypeBind + Default> {
    i: usize,
    p: PhantomData<T>,
}

impl<T: FieldTypeBind + Default + Eq> Producer<T> {
    pub fn set(&self, seq: &ElementSequencer, storage: &mut impl Storage<T>, value: T) {
        if let Some(index) = seq.get_index(&FieldKey::new::<T>(self.i)) {
            // TODO: Mark if the value actually changed.
            storage.set(index, value);
        }
    }
}

pub enum DependentKind {
    // The dependency will not be evaulated unless the component requests and then defers execution.
    // This sequence causes problems, but may be required.
    // This is currently required for short circuiting, but that may be outlined to a specific consumer.
    Lazy = 0,
    // The dependency will be evaulated only if the element is required to be invoked
    // by an active or trigger dependency.
    Required = 1,
    // The dependency will be checked for a delta if an upstream consumer requires the value.
    // The element is invoked only if one trigger dependency is either different, or a bool trigger returning true.
    Trigger = 2,
    // The dependency MUST be filled during an execution,
    // if the activation criteria is met the element will be invoked in the update cycle.
    // In general, a bool consumer should be be active on a field of this kind, along with
    // the other triggering criteria set as active only while the bool consumer is true.
    // This simulates an active flag.
    // Any active dependency will ALWAYS be evaluated, use it with caution.
    Active = 3,
}

pub struct ConsumerFlags {
    dependency: DependentKind,
}

impl ConsumerFlags {
    pub fn new() -> ConsumerFlags {
        ConsumerFlags {
            dependency: DependentKind::Required,
        }
    }
}

pub struct Consumer<T: FieldTypeBind + Default + Eq> {
    i: usize,
    p: PhantomData<T>,
    f: ConsumerFlags,
}

impl<T: FieldTypeBind + Default + Eq> Consumer<T> {
    pub fn bind(&self, seq: &mut ElementSequencer, producer: &Producer<T>) {
    }

    pub fn release(&self, seq: &mut ElementSequencer) {
    }

    pub fn get<'a>(&self, seq: &ElementSequencer, storage: &'a impl Storage<T>) -> Option<&'a T> {
        if let Some(index) = seq.get_index(&FieldKey::new::<T>(self.i)) {
            storage.get(index)
        } else {
            None
        }
    }
}
