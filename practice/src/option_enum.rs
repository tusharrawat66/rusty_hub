#[allow(unused_assignments)]
pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;

}


#[allow(unused_assignments)]
pub fn test_option_string() -> Option<String> {
    let mut opt1:Option<String> = None;
    opt1 = Some("This is a test".to_string());
    return opt1;
}


#[allow(unused_assignments)]
pub fn test_option_chartype() -> Option<CharacterType> {
    let mut chartype: Option<CharacterType> = None;
    chartype = Some(CharacterType::Mage);
    return  chartype;
}

#[allow(dead_code)]
pub enum CharacterType {
    Archer,
    Warrior,
    Mage
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self{
            CharacterType::Archer=> "Archer",
            CharacterType::Mage=> "Mage",
            CharacterType::Warrior=> "Warrior"
        }.to_string()        
    }
}