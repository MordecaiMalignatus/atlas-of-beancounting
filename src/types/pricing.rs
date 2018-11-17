

/// A price for an Item, relative to a currency, for example chaos orbs.
#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub currency_name: String,
    pub currency_id: u32,
    pub amount: f32,
}
