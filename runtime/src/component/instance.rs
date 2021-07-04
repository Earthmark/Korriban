use super::template::Template;
use crate::field::reservation::{ElementSequencer, ConsumerFlags};
use crate::prop::PropSet;
use std::collections::BTreeMap;

struct ExectuionContext<'a> {
    reserver: &'a crate::field::reservation::ElementSequencer,
}

struct InstanceTable<'t> {
    template: &'t super::template::Template,
    instances: Vec<Instance>,
}

impl<'t> InstanceTable<'t> {
    fn allocate(&mut self, r: &mut ElementSequencer) -> usize {
        let mut element = r.create_element();
        let producer = element.create_producer::<i32>(r);
        let consumer = element.create_consumer::<i32>(r, ConsumerFlags::new());
        0
    }

    fn execute(&self, index: usize, props: &PropSet) {
        if let Some(inst) = self.instances.get(index) {}
    }
}

struct Instance {
    index_map: IndexMapping,
}

impl Instance {
    fn execute(&self, props: &PropSet) {}
}

struct IndexMapping {
    maps: BTreeMap<i32, usize>,
}

impl IndexMapping {
    fn new(template: &Template) -> IndexMapping {
        IndexMapping {
            maps: BTreeMap::new(),
        }
    }

    fn get<T>(&self, index: i32) -> Option<&usize> {
        self.maps.get(&index)
    }
}
