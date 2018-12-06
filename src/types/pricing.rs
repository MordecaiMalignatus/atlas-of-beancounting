use types::poe_ninja::NinjaLineResponse;

/// A price for an Item, relative to a currency, for example chaos orbs.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Price {
    /// The name of the item that is being priced.
    pub name: String,
    pub chaos_equivalent: f32,
}


impl From<NinjaLineResponse> for Price {
    fn from(t: NinjaLineResponse) -> Price {
        match t.currencyTypeName {
            Some(x) => Price {
                name: x,
                chaos_equivalent: t.chaosEquivalent.unwrap_or(0.0),
            },
            None => match t.name {
                Some(x) => Price {
                    name: x,
                    chaos_equivalent: t.chaosValue.unwrap_or(0.0),
                },
                None => panic!(
                    "Poe.ninja delivered bad JSON, neither name nor currency_name are defined"
                ),
            },
        }
    }
}

#[derive(Debug)]
pub enum PriceMessage {
    Get { item: String },
    Response { item: String, price: Price },
    InvalidateCache,
    ShutDown
}
