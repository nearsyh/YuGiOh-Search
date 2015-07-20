extern crate string_cache;
extern crate tendril;
extern crate html5ever;

use std::collections::HashMap;
use std::borrow::Cow;
use std::default::Default;
use self::string_cache::namespace::{QualName};
use self::tendril::{StrTendril};
use html5ever::driver::{ParseResult};
use html5ever::tokenizer::Attribute;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText, AppendNode, AppendText};
use super::super::CardInfo;

pub struct CardInfoSink {
    next_id: usize,
    names: HashMap<usize, QualName>,
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

impl Default for CardInfoSink {
    fn default() -> Self {
        CardInfoSink {
            next_id: 0,
            names: HashMap::new(),
            values: vec![],
            in_header: false,
            in_data: false,
            temp: String::new(),
        }
    }
}

impl TreeSink for CardInfoSink {
    type Handle = usize;

    fn parse_error(&mut self, _msg: Cow<'static, str>) {}

    fn get_document(&mut self) -> usize { 0 }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) { }

    fn same_node(&self, x: usize, y: usize) -> bool { x == y }

    fn elem_name(&self, target: usize) -> QualName {
        self.names.get(&target).expect("not an element").clone()
    }

    fn create_element(&mut self, name: QualName, _attrs: Vec<Attribute>) -> usize {
        let id = self.get_id();
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

    fn create_comment(&mut self, _text: StrTendril) -> usize {
        let id = self.get_id();
        let name = self.names.get(&(id - 1)).unwrap().clone();
        self.names.insert(id, name);
        id
    }

    fn append(&mut self, parent: usize, child: NodeOrText<usize>) {
        match child {
            AppendNode(n)
                => {
                    if self.in_header && &(*self.elem_name(n).local) == "td" {
                        self.in_header = false;
                        self.values.push(self.temp.replace("\\n", ""));
                        println!("{} {}", self.values.len(), self.temp.replace("\\n", ""));
                        self.temp = String::new();
                    }
                    if self.in_data && &(*self.elem_name(n).local) == "tr" {
                        self.in_data = false;
                        self.values.push(self.temp.replace("\\n", ""));
                        println!("{} {}", self.values.len(), self.temp.replace("\\n", ""));
                        self.temp = String::new();
                    }
                },
            AppendText(t)
                => {
                    if self.in_header || self.in_data {
                        if &*self.elem_name(parent).local != "noscript" {
                            self.temp = self.temp.clone() + &t.escape_default();
                        }
                    }
                },
        }
    }

    fn append_before_sibling(&mut self,
            _sibling: usize,
            _new_node: NodeOrText<usize>) -> Result<(), NodeOrText<usize>> {
        Ok(())
    }
    fn append_doctype_to_document(&mut self,
                                  _name: StrTendril,
                                  _public_id: StrTendril,
                                  _system_id: StrTendril) {}
    fn add_attrs_if_missing(&mut self, _target: usize, _attrs: Vec<Attribute>) {}
    fn remove_from_parent(&mut self, _target: usize) { }
    fn reparent_children(&mut self, _node: usize, _new_parent: usize) { }
    fn mark_script_already_started(&mut self, _node: usize) { }
}

impl ParseResult for CardInfo {
    type Sink = CardInfoSink;
    fn get_result(sink : Self::Sink) -> Self {
        let mut ret = CardInfo::default();
        for i in 0..sink.values.len()/2 {
            match sink.values[2*i].escape_default().trim_right_matches(|c : char| c.is_whitespace()) {
                "English" => ret.name = Some(sink.values[2*i+1].clone()),
                "Attribute" => ret.attribute = Some(sink.values[2*i+1].clone()),
                "Types" => ret.types = Some(sink.values[2*i+1].clone()),
                "Level" => ret.level = Some(sink.values[2*i+1].clone()),
                "ATK/DEF" => ret.atkdef = Some(sink.values[2*i+1].clone()),
                "Card Number" => ret.card_num = Some(sink.values[2*i+1].clone()),
                "Passcode" => ret.passcode = Some(sink.values[2*i+1].clone()),
                "Card effect types" => ret.effect_types = Some(sink.values[2*i+1].clone()),
                "Materials" => ret.materials = Some(sink.values[2*i+1].clone()),
                "Fusion Materials" => ret.fusion_materials = Some(sink.values[2*i+1].clone()),
                "Rank" => ret.rank = Some(sink.values[2*i+1].clone()),
                "Ritual Spell Card required" => ret.ritual_spell = Some(sink.values[2*i+1].clone()),
                "Pendulum Scale" => ret.pendulum_scale = Some(sink.values[2*i+1].clone()),
                "Type" => ret.type_spell_trap = Some(sink.values[2*i+1].clone()),
                "Property" => ret.property = Some(sink.values[2*i+1].clone()),
                "Summoned by the effect of" => ret.summoned_by = Some(sink.values[2*i+1].clone()),
                "Limitation Text" => ret.limit_text = Some(sink.values[2*i+1].clone()),
                "Synchro Material" => ret.synchro_material = Some(sink.values[2*i+1].clone()),
                "Ritual Monster required" => ret.ritual_monster = Some(sink.values[2*i+1].clone()),
                _ => continue,
            }
            //println!("{} {}", sink.values[2*i], sink.values[2*i+1]);
        }
        //println!("{}", ret.name.clone().unwrap());
        ret
    }
}
