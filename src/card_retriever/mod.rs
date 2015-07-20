pub mod wikia_card_retriever;
mod database;

use std::collections::HashMap;

pub trait CardRetriever {
    fn retrieve_card_info() -> HashMap<String, CardInfo>;
}

#[test]
fn test() {
    let conn = database::DatabaseConnecter::connect("postgres://postgres@localhost/yugioh");
    conn.create_with_card_info(&wikia_card_retriever::WikiaCardRetriever::retrieve_card_info());
}

pub struct CardInfo {
    pub name: Option<String>,
    pub attribute: Option<String>,
    pub types: Option<String>,
    pub level: Option<String>,
    pub atkdef: Option<String>,
    pub card_num: Option<String>,
    pub passcode: Option<String>,
    pub effect_types: Option<String>,
    pub materials: Option<String>,
    pub fusion_materials: Option<String>,
    pub rank: Option<String>,
    pub ritual_spell: Option<String>,
    pub pendulum_scale: Option<String>,
    pub type_spell_trap: Option<String>,
    pub property: Option<String>,
    pub summoned_by: Option<String>,
    pub limit_text: Option<String>,
    pub synchro_material: Option<String>,
    pub ritual_monster: Option<String>,
}


impl Default for CardInfo {
    fn default() -> Self {
        CardInfo {
            name : None,
            attribute : None,
            types : None,
            level : None,
            atkdef : None,
            card_num : None,
            passcode : None,
            effect_types : None,
            materials : None,
            fusion_materials : None,
            rank : None,
            ritual_spell : None,
            pendulum_scale : None,
            type_spell_trap : None,
            property : None,
            summoned_by : None,
            limit_text : None,
            synchro_material : None,
            ritual_monster : None,
        }
    }
}
