mod structs;
use structs::gen3;

mod parsers;

pub fn parse(file_contents: Vec<u8>) -> Result<gen3::Gen3Save, String> {
    // TODO: detect gen

    gen3::Gen3Save::build(file_contents)
}
