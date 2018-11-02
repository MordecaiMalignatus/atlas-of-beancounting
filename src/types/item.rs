
#[derive(Debug)]
pub enum Item {
    Gear(Gear),
    Currency(Currency),
    DivinationCard(DivinationCard),
    Map(Map),
}

#[derive(Debug, PartialEq)]
pub struct Gear {
    pub name: String,
    pub rarity: ItemRarity,
    pub affixes: Vec<String>,
    pub sockets: String,
    pub item_level: u32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq)]
pub struct Currency {
    pub name: String,
    pub stack_size: StackSize,
    pub affixes: Vec<String>,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct DivinationCard {
    pub name: String,
    pub stack_size: StackSize,
    pub reward: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct Map {
    pub name: Option<String>,
    pub kind: String,
    pub tier: u32,
    pub item_quantity: u32,
    pub item_rarity: u32,
    pub quality: u32,
    pub pack_size: u32,
    pub affixes: Vec<String>,
    pub item_level: u32,
    pub rarity: ItemRarity,
}

#[derive(Debug, PartialEq)]
pub struct Requirements {
    pub level: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
}

#[derive(Debug, PartialEq)]
pub struct StackSize {
    pub current: u32,
    pub max: u32,
}

#[derive(Debug, PartialEq)]
pub enum ItemRarity {
    Currency,
    DivinationCard,
    Normal,
    Magical,
    Rare,
    Unique,
}

pub type Rest = String;

#[derive(Debug, PartialEq)]
pub enum KeyCapture {
    Capture(String, Rest),
    NoCapture(Rest),
}
