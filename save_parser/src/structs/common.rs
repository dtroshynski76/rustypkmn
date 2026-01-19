#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub enum Badges<T> {
    First(T),
    Second(T),
    Third(T),
    Fourth(T),
    Fifth(T),
    Sixth(T),
    Seventh(T),
    Eighth(T),
}

#[derive(Debug)]
pub struct Trainer<B> {
    pub name: String,
    pub id: u8,
    pub sid: u8,
    pub gender: Gender,
    pub hours: u16,
    pub minutes: u8,
    pub seconds: u8,
    pub rival_name: String,
    pub money: u32,
    pub coins: u16,
    pub badges: Vec<Badges<B>>,
}

#[derive(Debug)]
pub struct Items {}

#[derive(Debug)]
pub struct PartyPokemon {}

#[derive(Debug)]
pub struct PCPokemon {}

#[derive(Debug)]
pub struct Pokedex {}

pub struct OffsetSize {
    pub offset: usize,
    pub size_bytes: usize,
}

impl OffsetSize {
    pub fn sum(&self) -> usize {
        self.offset + self.size_bytes
    }
}
