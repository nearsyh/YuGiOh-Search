/*
extern crate lookup;

use lookup::card_retriever::card_retriever;
use lookup::card_retriever::card_retriever::Wikiacard_retriever;
*/

// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(str_escape)]

extern crate string_cache;
extern crate tendril;
extern crate html5ever;

use std::io;
use std::default::Default;
use std::collections::HashMap;
use std::borrow::Cow;
use string_cache::namespace::QualName;

use tendril::{ByteTendril, StrTendril, ReadExt};

use html5ever::{parse_to, one_input};
use html5ever::tokenizer::Attribute;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText, AppendNode, AppendText};

struct CardInfoSink {
    next_id: usize,
    names: HashMap<usize, QualName>,
    append: HashMap<usize, bool>,
    values: Vec<String>,
    in_header: bool,
    in_data: bool,
    temp: String,
}

impl CardInfoSink {
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl TreeSink for CardInfoSink {
    type Handle = usize;

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        println!("Parse error: {}", msg);
    }

    fn get_document(&mut self) -> usize {
        0
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        println!("Set quirks mode to {:?}", mode);
    }

    fn same_node(&self, x: usize, y: usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: usize) -> QualName {
        self.names.get(&target).expect("not an element").clone()
    }

    fn create_element(&mut self, name: QualName, _attrs: Vec<Attribute>) -> usize {
        let id = self.get_id();
        println!("Created {:?} as {}", name, id);
        self.append.insert(id, false);
        for attr in &_attrs {
            match &(*attr.value) {
                "cardtablerowheader" => { self.in_header = true; break; },
                "cardtablerowdata" => { self.in_data = true; break; },
                _ => continue,
            }
        }
        self.names.insert(id, name);
        id
    }

    fn create_comment(&mut self, text: StrTendril) -> usize {
        let id = self.get_id();
        println!("Created comment \"{}\" as {}", text.escape_default(), id);
        id
    }

    fn append(&mut self, parent: usize, child: NodeOrText<usize>) {
        match child {
            AppendNode(n)
                => {
                    if &(*self.elem_name(n).local) == "td" {
                        self.in_header = false;
                        self.values.push(self.temp.clone());
                        self.temp = String::new();
                    }
                    if &(*self.elem_name(n).local) == "tr" {
                        self.in_data = false;
                        self.values.push(self.temp.clone());
                        self.temp = String::new();
                    }
                    println!("Append node to {}: \"{}\"", parent, n);
                },
            AppendText(t)
                => {
                    if self.in_header || self.in_data {
                        if &*self.elem_name(parent).local != "noscript" {
                            self.temp = self.temp.clone() + &t.escape_default();
                        }
                    }
                    println!("Append text to {}: \"{}\"", parent, t.escape_default());
                },
        }
    }

    fn append_before_sibling(&mut self,
            sibling: usize,
            new_node: NodeOrText<usize>) -> Result<(), NodeOrText<usize>> {
        match new_node {
            AppendNode(n)
                => println!("Append node {} before {}", n, sibling),
            AppendText(t)
                => println!("Append text before {}: \"{}\"", sibling, t.escape_default()),
        }

        // `sibling` will have a parent unless a script moved it, and we're
        // not running scripts.  Therefore we can aways return `Ok(())`.
        Ok(())
    }

    fn append_doctype_to_document(&mut self,
                                  name: StrTendril,
                                  public_id: StrTendril,
                                  system_id: StrTendril) {
        println!("Append doctype: {} {} {}", name, public_id, system_id);
    }

    fn add_attrs_if_missing(&mut self, target: usize, attrs: Vec<Attribute>) {
        println!("Add missing attributes to {}:", target);
        for attr in attrs.into_iter() {
            println!("    {:?} = {}", attr.name, attr.value);
        }
    }

    fn remove_from_parent(&mut self, target: usize) {
        println!("Remove {} from parent", target);
    }

    fn reparent_children(&mut self, node: usize, new_parent: usize) {
        println!("Move children from {} to {}", node, new_parent);
    }

    fn mark_script_already_started(&mut self, node: usize) {
        println!("Mark script {} as already started", node);
    }
}

fn main() {
    let mut sink = CardInfoSink {
        next_id: 1,
        names: HashMap::new(),
        append: HashMap::new(),
        values: vec![],
        in_header: false,
        in_data: false,
        temp: String::new(),
    };

    let mut input = ByteTendril::new();
    io::stdin().read_to_tendril(&mut input).unwrap();
    let input = input.try_reinterpret().unwrap();
    sink = parse_to(sink, one_input(input), Default::default());

    for value in &sink.values {
        println!("{}", value);
    }
}
