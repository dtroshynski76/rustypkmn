use crate::structs::common::{
    Badges, Gender, Items, OffsetSize, PCPokemon, PartyPokemon, Pokedex, Trainer,
};
use crate::structs::gen3::{Gen3Badges, Gen3Save, Gen3Section};

const SAVE_A_OFFSET: OffsetSize = OffsetSize {
    offset: 0x000000, // 0
    size_bytes: 57344,
};
const SAVE_B_OFFSET: OffsetSize = OffsetSize {
    offset: 0x00E000, // 57344
    size_bytes: 57344,
};
pub const SECTION_SIZE: usize = 4096;
const SECTION_SIGNATURE: OffsetSize = OffsetSize {
    offset: 0x0FF8,
    size_bytes: 4,
};
const SIGNATURE_VALUE: u32 = 0x08012025;
const SECTION_DATA: OffsetSize = OffsetSize {
    offset: 0x0000,
    size_bytes: 3968,
};
const SECTION_ID: OffsetSize = OffsetSize {
    offset: 0x0FF4,
    size_bytes: 2,
};
const SECTION_CHECKSUM: OffsetSize = OffsetSize {
    offset: 0x0FF6,
    size_bytes: 2,
};
const SECTION_INDEX: OffsetSize = OffsetSize {
    offset: 0x0FFC,
    size_bytes: 4,
};
const HALL_OF_FAME_OFFSET: OffsetSize = OffsetSize {
    offset: 0x01C000, // 114688
    size_bytes: 8192,
};
const MYSTERY_GIFT_OFFSET: OffsetSize = OffsetSize {
    offset: 0x01E000, // 122880
    size_bytes: 4096,
};
const RECORDED_BATTLE_OFFSET: OffsetSize = OffsetSize {
    offset: 0x01F000, // 126976
    size_bytes: 4096,
};

impl Gen3Save {
    pub fn build(content: Vec<u8>) -> Result<Gen3Save, String> {
        Self::verify_content_length(&content)?;

        let most_recent_save = Self::get_most_recent_save(&content)?;

        let sections: Vec<Gen3Section> = most_recent_save
            .chunks(SECTION_SIZE)
            .map(|sec: &[u8]| Gen3Section::build(sec))
            .collect::<Result<Vec<Gen3Section>, String>>()?;

        println!("sections: {:?}", sections);
        println!("size: {}", content.len());

        Ok(Gen3Save {
            trainer: Trainer {
                name: String::from("test name"),
                id: 1,
                sid: 2,
                badges: Vec::from([Badges::First(Gen3Badges::Boulder)]),
                coins: 0,
                gender: Gender::Male,
                hours: 0,
                minutes: 1,
                seconds: 0,
                money: 0,
                rival_name: String::from("Blue"),
            },
            items: Items {},
            party_pokemon: PartyPokemon {},
            pc_pokemon: PCPokemon {},
            pokedex: Pokedex {},
        })
    }

    fn verify_content_length(content: &[u8]) -> Result<(), String> {
        if content.len() != 131_072 {
            // wrong file size
            // size must be 131072 bytes (or 128 kibibytes/KB)
            return Err(String::from(
                "File size is wrong. File must be 128 KB (131,072 bytes).",
            ));
        }

        Ok(())
    }

    fn get_most_recent_save(content: &[u8]) -> Result<Vec<u8>, String> {
        let game_save_a = &content[SAVE_A_OFFSET.offset..SAVE_A_OFFSET.sum()];
        let game_save_b = &content[SAVE_B_OFFSET.offset..SAVE_B_OFFSET.sum()];

        let save_a_last_section_index = &game_save_a[game_save_a.len() - 4..game_save_a.len()]
            .iter()
            .sum::<u8>();
        let save_b_last_section_index = &game_save_b[game_save_b.len() - 4..game_save_b.len()]
            .iter()
            .sum::<u8>();

        let most_recent_save = if save_a_last_section_index > save_b_last_section_index {
            Vec::from(game_save_a)
        } else {
            Vec::from(game_save_b)
        };

        if most_recent_save.len() != SAVE_A_OFFSET.size_bytes {
            return Err(format!(
                "Expected a save game slot of size {} but got {} instead",
                SAVE_A_OFFSET.size_bytes,
                most_recent_save.len(),
            ));
        }

        Ok(most_recent_save)
    }
}

impl Gen3Section {
    pub fn build(section: &[u8]) -> Result<Gen3Section, String> {
        if section.len() != SECTION_SIZE {
            return Err(format!(
                "Expected section size of {} but got {} instead",
                SECTION_SIZE,
                section.len()
            ));
        }

        let data: &[u8; 3968] = &section[SECTION_DATA.offset..SECTION_DATA.sum()]
            .try_into()
            .unwrap();
        let section_id: &[u8; 2] = &section[SECTION_ID.offset..SECTION_ID.sum()]
            .try_into()
            .unwrap();
        let checksum: &[u8; 2] = &section[SECTION_CHECKSUM.offset..SECTION_CHECKSUM.sum()]
            .try_into()
            .unwrap();
        let signature = Self::validate_section_signature(section)?;
        let save_index: &[u8; 4] = &section[SECTION_INDEX.offset..SECTION_INDEX.sum()]
            .try_into()
            .unwrap();

        // TODO: parse section data into struct based on its id

        Ok(Gen3Section {
            data: *data,
            section_id: *section_id,
            checksum: *checksum,
            signature: *signature,
            save_index: *save_index,
        })
    }

    fn validate_section_signature(save_data: &[u8]) -> Result<&[u8; 4], String> {
        let signature = &save_data[SECTION_SIGNATURE.offset..SECTION_SIGNATURE.sum()];
        let num = u32::from_le_bytes(signature.try_into().unwrap());

        if num == SIGNATURE_VALUE {
            Ok(signature.try_into().unwrap())
        } else {
            Err(String::from("Section has invalid signature"))
        }
    }
}
