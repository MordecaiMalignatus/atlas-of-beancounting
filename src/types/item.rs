
#[derive(Debug)]
pub struct Item {
    pub description: String,
    pub name: String,
    pub affixes: Vec<String>,
    pub sockets: Option<String>,
    pub item_level: u32,
    pub attribute_requirements: Option<AttributeRequirements>,
}

#[derive(Debug)]
pub struct AttributeRequirements {
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
}
