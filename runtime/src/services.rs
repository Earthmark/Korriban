
trait InjectSource {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct Dep1 {
        
    }
    
    struct Dep2 {
        d1: Rc<Dep1>,
    }

    struct Injectable {
        d1: Rc<Dep1>,
        d2: Rc<Dep2>,
    }

    struct Container {
        d1: Rc<Dep1>,
        d2: Rc<Dep2>,
        i: Rc<Injectable>,
    }

    #[test]
    fn get_size() {
        let d1 = Rc::new(Dep1{});
        let d2 = Rc::new(Dep2{
            d1: d1.clone(),
        });
        let i = Rc::new(Injectable{
            d1: d1.clone(),
            d2: d2.clone(),
        });
        Container {
            d1,
            d2,
            i
        };
    }

}
