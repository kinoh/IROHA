extern crate typed_arena;

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::collections::HashMap;
use std::option::Option;
use self::typed_arena::Arena;

trait Entity {
    fn id(&self) -> usize;
}

pub struct ConceptData<'a> {
    pub id: usize,
    parent: RefCell<Option<Concept<'a>>>,
    children: RefCell<Vec<Concept<'a>>>,
    scheme: RefCell<Option<Concept<'a>>>
}
type Concept<'a> = &'a ConceptData<'a>;

impl<'a> Entity for ConceptData<'a> {
    fn id(&self) -> usize {
        self.id
    }
}

impl<'a> ConceptData<'a> {
    fn new(id: usize) -> ConceptData<'a> {
        ConceptData {
            id: id,
            parent: RefCell::new(None),
            children: RefCell::new(vec![]),
            scheme: RefCell::new(None)
        }
    }

    pub fn parent(&self) -> Ref<Option<Concept<'a>>> {
        self.parent.borrow()
    }

    pub fn children(&self) -> Ref<Vec<Concept<'a>>> {
        self.children.borrow()
    }

    pub fn scheme(&self) -> Ref<Option<Concept<'a>>> {
        self.scheme.borrow()
    }

    fn heritages(&self) -> Vec<Concept<'a>> {
        let mut res = vec![];
        let mut n = Some(self);
        while let Some(m) = n {
            res.extend(m.children().iter());
            n = m.scheme().clone();
        }
        res
    }
}

impl<'a> PartialEq for ConceptData<'a> {
    fn eq(&self, other: &ConceptData<'a>) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for ConceptData<'a> {}

pub struct Mind<'a> {
    arena: &'a Arena<ConceptData<'a>>,
    pub notions: Vec<Concept<'a>>,
    activator: HashMap<String, Concept<'a>>,
    conscious: VecDeque<Concept<'a>>
}

impl<'a> Mind<'a> {
    pub fn new(arena: &'a Arena<ConceptData<'a>>) -> Mind<'a> {
        Mind {
            arena: arena,
            notions: vec![],
            activator: HashMap::new(),
            conscious: VecDeque::new()
        }
    }

    pub fn know(&mut self) -> Concept<'a> {
        let notion = self.arena.alloc(ConceptData::new(self.notions.len()));
        self.notions.push(notion);
        notion
    }

    pub fn know_child(&mut self, notion: Concept<'a>) -> Concept<'a> {
        let child = self.know();
        child.parent.replace(Some(notion));
        notion.children.borrow_mut().push(child);
        child
    }

    pub fn define_as(&self, notion: Concept<'a>, scheme: Concept<'a>) {
        notion.scheme.replace(Some(scheme));
    }

    pub fn elaborate(&mut self, notion: Concept<'a>) -> Concept<'a> {
        let elaboration = self.know();
        elaboration.scheme.replace(Some(notion));
        elaboration
    }

    pub fn ground(&mut self, notion: Concept<'a>, key: String) {
        self.activator.insert(key, notion);
    }

    pub fn receive(&mut self, word: String) {
        match self.activator.get(&word) {
            Some(receptor) => {
                let index = self.conscious.iter().enumerate().find(|n| n.1 == receptor).map(|n| n.0);
                match index {
                    Some(i) => self.conscious.swap(i, 0),
                    None => self.conscious.push_front(receptor)
                }
            },
            None => ()
        }
    }

    pub fn collect_activators(&self, notion: Concept<'a>) -> Vec<String> {
        let mut res = vec![];
        for (a, n) in self.activator.iter() {
            if *n == notion {
                res.push(a.clone());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arena = Arena::new();
        let mut mind = Mind::new(&arena);

        let existence = mind.know();

        let verb = mind.know();
        let subject = mind.know_child(verb);

        mind.define_as(subject, existence);

        let walk = mind.elaborate(verb);
        mind.ground(walk, "walk".to_string());

        mind.receive("run".to_string());
        assert_eq!(mind.conscious.len(), 0);

        mind.receive("walk".to_string());
        assert_eq!(mind.conscious.len(), 1);
    }
}
