use std::collections::HashMap;
use cardInfo::CardInfo;

pub trait CardRetriever {
    fn retrieveCardInfo() -> HashMap<String, CardInfo>;
}
