extern crate lookup;

use lookup::wikiaCardRetriever::WikiaCardRetriever;
use lookup::cardRetriever::CardRetriever;

fn main() {
    WikiaCardRetriever::retrieveCardInfo();
    println!("Hello, world!");
}
