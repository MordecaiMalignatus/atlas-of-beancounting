
#[derive(Debug)]
pub enum Item {
    Gear {
        name: String,
        rarity: ItemRarity,
        affixes: Vec<String>,
        sockets: String,
        item_level: u32,
        requirements: Requirements,
    },
    Currency {
        name: String,
        stack_size: StackSize,
        affixes: Vec<String>,
        description: String,
    },
    DivinationCard {
        name: String,
        stack_size: StackSize,
        reward: String,
        description: String,
    },
    Map {
        name: String,
        tier: u32,
        affixes: Vec<String>,
        item_level: u32,
        rarity: ItemRarity,
    },
}

#[derive(Debug)]
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
