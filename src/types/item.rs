
#[derive(Debug)]
pub struct Item {
    pub description: String,
    pub name: String,
    pub rarity: ItemRarity,
    pub affixes: Vec<String>,
    pub sockets: Option<String>,
    pub item_level: u32,
    pub requirements: Option<Requirements>,
}

#[derive(Debug)]
pub struct Requirements {
    pub level: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
}

#[derive(Debug, PartialEq)]
pub enum ItemRarity {
    Currency,
    Normal,
    Magical,
    Rare,
    Unique,
}
