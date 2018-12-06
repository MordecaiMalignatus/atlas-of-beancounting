/// TODO: Get the list of this from the POE API:
/// https://www.pathofexile.com/developer/docs/api-resource-leagues
pub const CURRENT_LEAGUE: &str = "Standard";

pub const IS_DEBUG: bool = true;

/// URLs for poe.ninja. These need to have the `{}` `replace`d with the league
/// name before use.  before use. These are overspecific because the API is
/// locked down pretty tightly -- These are the endpoints that work and I
/// haven't gotten any others to work.
pub const POE_NINJA_ENDPOINT_TEMPLATES: &[&str] = &[
    "https://poe.ninja/api/data/currencyoverview?league={}&type=Currency",
    "https://poe.ninja/api/data/currencyoverview?league={}&type=Fragment",
    "https://poe.ninja/api/data/itemoverview?league={}&type=Fossil",
    "https://poe.ninja/api/data/itemoverview?league={}&type=Resonator",
    "https://poe.ninja/api/data/itemoverview?league={}&type=Essence",
    "https://poe.ninja/api/data/itemoverview?league={}&type=DivinationCard",
    "https://poe.ninja/api/data/itemoverview?league={}&type=Prophecy",
    "https://poe.ninja/api/data/itemoverview?league={}&type=SkillGem",
    "https://poe.ninja/api/data/itemoverview?league={}&type=BaseType",
    "https://poe.ninja/api/data/itemoverview?league={}&type=HelmetEnchant",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueMap",
    "https://poe.ninja/api/data/itemoverview?league={}&type=Map",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueJewel",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueFlask",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueWeapon",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueArmour",
    "https://poe.ninja/api/data/itemoverview?league={}&type=UniqueAccessory",
];
