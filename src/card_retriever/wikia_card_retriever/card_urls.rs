use rustc_serialize::{Decodable, Decoder};

pub struct CardUrl {
    _id: i64,
    pub title : String,
    pub url : String,
    _ns : i64,
}

pub struct CardUrls {
    pub items : Vec<CardUrl>,
    pub basepath : String,
    pub offset : Option<String>,
}

impl Decodable for CardUrl {
    fn decode<D : Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_struct("CardUrl", 4, |decoder| {
            Ok(CardUrl {
                _id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
                title: try!(decoder.read_struct_field("title", 1, |decoder| Decodable::decode(decoder))),
                url: try!(decoder.read_struct_field("url", 2, |decoder| Decodable::decode(decoder))),
                _ns: try!(decoder.read_struct_field("ns", 3, |decoder| Decodable::decode(decoder))),
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
