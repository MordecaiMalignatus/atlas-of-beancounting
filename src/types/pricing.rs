use types::poe_ninja::NinjaLineResponse;

/// A price for an Item, relative to a currency, for example chaos orbs.
#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub currency_name: String,
    pub chaos_equivalent: f32,
}

impl From<NinjaLineResponse> for Price {
    fn from(t: NinjaLineResponse) -> Price {
        Price {
            currency_name: t.currencyTypeName,
            chaos_equivalent: t.chaosEquivalent,
        }
    }
}

pub enum PriceMessage {
    Get { item: String },
    Response { item: String, price: Price },
}
