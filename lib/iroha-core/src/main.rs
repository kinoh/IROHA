extern crate iroha_core;
extern crate typed_arena;

use std::fs::File;
use typed_arena::Arena;

use iroha_core::dumper::dump;
use iroha_core::mind::Mind;

fn main() {
    let arena = Arena::new();
    let mut mind = Mind::new(&arena);

    let existence = mind.know();
    mind.ground(existence, "person".to_string());

    let verb = mind.know();
    let subject = mind.know_child(verb);

    mind.ground(verb, "do".to_string());

    mind.define_as(subject, existence);

    let walk = mind.elaborate(verb);
    mind.ground(walk, "walk".to_string());

    let mut out = File::create("out/mind.dot").unwrap();
    dump(&mut out, mind);
}
