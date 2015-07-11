use std::collections::HashMap;
use cardInfo::CardInfo;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::{json, Decodable, Decoder};
use cardRetriever::CardRetriever;
use std::sync::Arc;

pub struct WikiaCardRetriever;


struct CardUrl {
    id: i64,
    title : String,
    url : String,
    ns : i64,
}

struct CardUrls {
    items : Vec<CardUrl>,
    basepath : String,
    offset : Option<String>,
}

impl Decodable for CardUrl {
    fn decode<D : Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_struct("CardUrl", 4, |decoder| {
            Ok(CardUrl {
                id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
                title: try!(decoder.read_struct_field("title", 1, |decoder| Decodable::decode(decoder))),
                url: try!(decoder.read_struct_field("url", 2, |decoder| Decodable::decode(decoder))),
                ns: try!(decoder.read_struct_field("ns", 3, |decoder| Decodable::decode(decoder))),
            })
        })
    }
}

impl Decodable for CardUrls {
    fn decode<D : Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_struct("CardUrls", 3, |decoder| {
            Ok(CardUrls {
                items: try!(decoder.read_struct_field("items", 0, |decoder| Decodable::decode(decoder))),
                basepath: try!(decoder.read_struct_field("basepath", 1, |decoder| Decodable::decode(decoder))),
                offset: try!(decoder.read_struct_field("offset", 2, |decoder| Decodable::decode(decoder))),
            })
        })
    }
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
        return ret;
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
}

impl CardRetriever for WikiaCardRetriever {
    fn retrieveCardInfo() -> HashMap<String, CardInfo> {
        let mut ret = HashMap::<String, CardInfo>::new();
        let card_urls = WikiaCardRetriever::retrieve_card_url();
        for card_url in &card_urls.items {
            
        }
        return ret;
    }
}


#[test]
fn testJson(){
    WikiaCardRetriever::retrieve_card_url();
}
