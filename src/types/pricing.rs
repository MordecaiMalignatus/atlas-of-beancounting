

/// A price for an Item, relative to a currency, for example chaos orbs.
pub struct Price {
    pub currency_name: String,
    pub currency_id: u32,
    pub amount: f32,
}
