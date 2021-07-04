use crate::prop::PropSet;

pub trait Element {
    fn update(&self, src: &PropSet, dest: &mut PropSet);
}

pub struct Space {
    props: PropSet,
    elements: Vec<Box<dyn Element>>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            props: PropSet::new(),
            elements: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let mut dest = self.props.clone();

        for elem in &self.elements {
            elem.update(&self.props, &mut dest);
        }

        self.props = dest;
    }

    pub fn add_element(&mut self, elem: Box<dyn Element>) {
        self.elements.push(elem);
    }
}
