extern crate dot;

use mind::Mind;
use std::borrow::Cow;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::Write;

type Nd = (usize, Option<String>);

#[derive(Clone)]
enum EdgeType {
    HasA,
    ElaboratedTo,
    Grounds,
}

type Ed = (usize, usize, EdgeType);

struct Graph {
    nodes: Vec<Nd>,
    edges: Vec<Ed>
}

pub fn dump<W: Write>(output: &mut W, mind: Mind) -> io::Result<()> {
    let graph = Graph::new(mind);
    dot::render(&graph, output)
}

impl Graph {
    fn new(mind: Mind) -> Graph {
        let mut nodes = vec![];
        let mut edges = vec![];

        let mut count = mind.notions.len();

        for n in &mind.notions {
            nodes.push((n.id, None));

            match *n.scheme() {
                Some(scheme) => edges.push((scheme.id, n.id, EdgeType::ElaboratedTo)),
                None => ()
            }

            for child in n.children().iter() {
                edges.push((n.id, child.id, EdgeType::HasA));
            }

            for activator in &mind.collect_activators(n) {
                let id = count;
                count += 1;

                nodes.push((id, Some(activator.clone())));
                edges.push((n.id, id, EdgeType::Grounds));
            }
        }

        Graph { nodes: nodes, edges: edges }
    }

    fn node_by_id(&self, id: usize) -> Nd {
        for n in &self.nodes {
            let (i, _) = *n;
            if i == id {
                return n.clone();
            }
        }
        panic!();
    }
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("Mind").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        let &(ref id, _) = n;
        dot::Id::new(format!("N{}", *id)).unwrap()
    }

    fn node_label(&'a self, n: &Nd) -> dot::LabelText<'a> {
        let &(_, ref t) = n;
        let s = match *t {
            Some(ref label) => label.clone(),
            None => "".to_string(),
        };
        dot::LabelText::LabelStr(Cow::Owned(s))
    }

    fn node_shape(&'a self, n: &Nd) -> Option<dot::LabelText<'a>> {
        let &(_, ref t) = n;
        let s = match *t {
            Some(_) => "box",
            None => "ellipse",
        }.to_string();
        Some(dot::LabelText::LabelStr(Cow::Owned(s)))
    }

    fn edge_label(&'a self, e: &Ed) -> dot::LabelText<'a> {
        let &(_, _, ref t) = e;
        let s = match *t {
            EdgeType::ElaboratedTo => "elaborated-to",
            EdgeType::HasA => "has-a",
            EdgeType::Grounds => "grounds",
        }.to_string();
        dot::LabelText::LabelStr(Cow::Owned(s))
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        Cow::Owned(self.nodes.clone())
    }

    fn edges(&self) -> dot::Edges<'a, Ed> {
        Cow::Owned(self.edges.clone())
    }

    fn source(&self, e: &Ed) -> Nd { let &(s, _, _) = e; self.node_by_id(s) }
    fn target(&self, e: &Ed) -> Nd { let &(_, t, _) = e; self.node_by_id(t) }
}
