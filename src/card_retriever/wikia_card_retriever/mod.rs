extern crate string_cache;
extern crate tendril;
extern crate html5ever;

mod card_urls;
mod card_info_sink;

use std::collections::HashMap;
use super::CardInfo;
use std::io::Read;
use std::thread;
use std::sync::{Arc, mpsc};
use hyper::Client;
use std::iter::FromIterator;
use hyper::header::Connection;
use rustc_serialize::{json};
use card_retriever::CardRetriever;
use self::tendril::{StrTendril, SliceExt};
use html5ever::{one_input};
use html5ever::driver::{parse};

use self::card_urls::CardUrls;

pub struct WikiaCardRetriever;

fn chunk<T, U>(data: U, num: usize) -> Vec<U>
    where U: IntoIterator<Item=T>,
    U: FromIterator<T>,
    <U as IntoIterator>::IntoIter: ExactSizeIterator
{
    let mut iter = data.into_iter();
    let iter = iter.by_ref();
    let chunk_len = (iter.len() / num) as usize + 1;
    let mut chunks = Vec::new();
    for _ in 0..num {
        chunks.push(iter.take(chunk_len).collect())
    }
    chunks
}

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

    fn get_card_info(url : &str, client : &Client) -> CardInfo {
        println!("Url is {}", url);
        let mut res = client.get(url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("get response");
        Self::parse_html_to_card_info(&body)
    }

    fn parse_html_to_card_info(html : &str) -> CardInfo {
        let input : StrTendril = html.to_tendril();
        parse(one_input(input), Default::default())
    }

    fn retrieve_card_info_by_url(urls: &HashMap<String, String>) -> HashMap<String, CardInfo> {
        let mut ret = HashMap::<String, CardInfo>::new();
        let mut count = 0;
        let client = Client::new();

        for (name, url) in urls {
            count += 1;
            println!("{} : start to get {}", count, name.clone());
            let info = WikiaCardRetriever::get_card_info(url, &client);
            if info.name == None {
                continue;
            }
            ret.insert(name.clone(), info);
        }
        ret
    }

    fn retrieve_card_info_concurrent() -> HashMap<String, CardInfo> {
        let mut ret = HashMap::<String, CardInfo>::new();
        let thread_num = 4;
        let card_urls = Arc::new(chunk(WikiaCardRetriever::retrieve_card_url(), thread_num));
        let client = Client::new();
        let (tx, rx) = mpsc::channel();

        for i in 0..thread_num {
            let tx = tx.clone();
            let card_urls = card_urls.clone();
            thread::spawn(move || {
                tx.send(WikiaCardRetriever::retrieve_card_info_by_url(&card_urls[i]));
            });
        }

        for i in 0..thread_num {
            for (key, value) in rx.recv().unwrap().into_iter() {
                ret.insert(key, value);
            }
        }
        println!("finished");
        ret
    }
}

impl CardRetriever for WikiaCardRetriever {
    fn retrieve_card_info() -> HashMap<String, CardInfo> {
        //WikiaCardRetriever::retrieve_card_info_by_url(&WikiaCardRetriever::retrieve_card_url())
        WikiaCardRetriever::retrieve_card_info_concurrent()
    }
}
