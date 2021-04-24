use std::collections::BTreeMap;
use super::template::Template;
use crate::prop::PropSet;

struct InstanceTable<'t> {
    template: &'t super::template::Template,
    instances: Vec<Instance>,
}

impl<'t> InstanceTable<'t> {
    fn allocate(&mut self) -> usize {

        0
    }

    fn execute(&self, index: usize, props: &PropSet) {
        if let Some(inst) = self.instances.get(index) {

        }
    }
}

struct Instance {
    index_map: IndexMapping,
}

impl Instance {
    fn execute(&self, props: &PropSet) {
        
    }
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
