use crate::structs::common::{Items, PCPokemon, PartyPokemon, Pokedex};

use super::common::Trainer;

#[derive(Debug)]
pub struct Gen3Save {
    pub trainer: Trainer<Gen3Badges>,
    pub items: Items,
    pub party_pokemon: PartyPokemon,
    pub pc_pokemon: PCPokemon,
    pub pokedex: Pokedex,
}

#[derive(Debug)]
pub struct Gen3Section {
    // TODO: change types to more meaningful struct types
    pub data: [u8; 3968],
    pub section_id: [u8; 2],
    pub checksum: [u8; 2],
    pub signature: [u8; 4],
    pub save_index: [u8; 4],
}

#[derive(Debug)]
pub enum Gen3Badges {
    Boulder,
    Cascade,
    Thunder,
    Rainbow,
    Soul,
    Marsh,
    Volcano,
    Earth,
}
