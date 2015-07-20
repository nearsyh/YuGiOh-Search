extern crate postgres;

use super::CardInfo;
use std::collections::HashMap;
use self::postgres::{Connection, SslMode};

pub struct DatabaseConnecter {
    conn : Connection,
}

impl DatabaseConnecter {
    pub fn connect(url : &str) -> Self {
        DatabaseConnecter {
            conn : Connection::connect(url, &SslMode::None).unwrap(),
        }
    }

    pub fn create_with_card_info(&self, card_infos : &HashMap<String, CardInfo>) {
        self.conn.execute("CREATE TABLE Card (
            name              TEXT NOT NULL,
            attribute         TEXT,
            types             TEXT,
            level             TEXT,
            atkdef            TEXT,
            cardnum           TEXT,
            passcode          TEXT,
            effectTypes       TEXT,
            materials         TEXT,
            fusionMaterials   TEXT,
            rank              TEXT,
            ritualSpell       TEXT,
            pendulumScale     TEXT,
            typeSpellTrap     TEXT,
            property          TEXT,
            summonedBy        TEXT,
            limitText         TEXT,
            synchroMaterial   TEXT,
            ritualMonster     TEXT
        )", &[]);

        let stmt = self.conn.prepare("INSERT INTO Card
            (name, attribute, types, level, atkdef, cardnum, passcode, effectTypes, materials,
            fusionMaterials, rank, ritualSpell, pendulumScale, typeSpellTrap, property, summonedBy,
            limitText, synchroMaterial, ritualMonster)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)").unwrap();

        //let mut count = 10;

        for (_name, card_info) in card_infos {
            stmt.execute(&[
                &card_info.name,
                &card_info.attribute,
                &card_info.types,
                &card_info.level,
                &card_info.atkdef,
                &card_info.card_num,
                &card_info.passcode,
                &card_info.effect_types,
                &card_info.materials,
                &card_info.fusion_materials,
                &card_info.rank,
                &card_info.ritual_spell,
                &card_info.pendulum_scale,
                &card_info.type_spell_trap,
                &card_info.property,
                &card_info.summoned_by,
                &card_info.limit_text,
                &card_info.synchro_material,
                &card_info.ritual_monster
            ]).unwrap();
            /*
            count -= 1;
            if count <= 0 {
                break;
            }
            */
        }
    }
}
