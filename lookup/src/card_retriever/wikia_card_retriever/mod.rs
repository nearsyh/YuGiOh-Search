extern crate string_cache;
extern crate tendril;
extern crate html5ever;

mod card_urls;
mod card_info_sink;

use std::collections::HashMap;
use super::CardInfo;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::{json};
use card_retriever::CardRetriever;
use self::tendril::{StrTendril, SliceExt};
use html5ever::{one_input};
use html5ever::driver::{parse};

use self::card_urls::CardUrls;

pub struct WikiaCardRetriever;

impl WikiaCardRetriever {
    fn retrieve_card_url() -> HashMap<String, String> {
        let baseurl = "http://yugioh.wikia.com/api/v1/Articles/List?category=TCG_cards&limit=5000&namespaces=0".to_string();
        let mut url = baseurl.clone();
        let mut ret = HashMap::<String, String>::new();
        loop {
            println!("Url : {}", url);
            let offset = Self::retrieve_card_url_part(&mut ret, &url);
            match offset {
                Some(ref off) => url = baseurl.clone() + "&offset=" + off,
                None => break,
            };
        }
        ret
    }

    fn retrieve_card_url_part(ret : &mut HashMap<String, String>, url : &str) -> Option<String> {
        let card_urls_part : CardUrls = json::decode(&WikiaCardRetriever::get_json_response(url)).unwrap();
        let basepath = card_urls_part.basepath.clone();
        for item in &card_urls_part.items {
            ret.insert(item.title.clone(), basepath.clone() + &item.url);
        }
        card_urls_part.offset
    }

    fn get_json_response(url : &str) -> String {
        let client = Client::new();
        let mut res = client.get(url).header(Connection::close()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn get_card_info(url : &str) -> CardInfo {
        let client = Client::new();
        let mut res = client.get(url).header(Connection::close()).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        Self::parse_html_to_card_info(&body)
    }

    fn parse_html_to_card_info(html : &str) -> CardInfo {
        let input : StrTendril = html.to_tendril();
        parse(one_input(input), Default::default())
    }
}

impl CardRetriever for WikiaCardRetriever {
    fn retrieve_card_info() -> HashMap<String, CardInfo> {
        let mut ret = HashMap::<String, CardInfo>::new();
        for (name, url) in &WikiaCardRetriever::retrieve_card_url() {
            println!("{}", name);
            ret.insert(name.clone(), WikiaCardRetriever::get_card_info(url));
            break;
        }
        ret
    }
}


#[test]
fn testJson(){
    WikiaCardRetriever::retrieve_card_info();
}
