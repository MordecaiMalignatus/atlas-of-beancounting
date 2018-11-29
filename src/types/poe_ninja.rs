//! This module is for the types that end up getting deserialized from
//! Poe.ninja. They're not necessarily the types used here, but needed as
//! deserialization target. Snake case is ignored because the names matter for
//! deserialization.

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NinjaCurrencyOverviewResponse {
    pub lines: Vec<NinjaLineResponse>,
    pub currencyDetails: Option<Vec<NinjaDetails>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NinjaLineResponse {
    // One of the two has to be present, one for currencies, one for the rest.
    pub currencyTypeName: Option<String>,
    pub name: Option<String>,

    pub pay: Option<NinjaPrice>,
    pub receive: Option<NinjaPrice>,

    // First is for currencies, latter for items.
    pub chaosEquivalent: Option<f32>,
    pub chaosValue: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NinjaPrice {
    pub id: u32,
    pub league_id: u32,
    pub pay_currency_id: u32,
    pub get_currency_id: u32,
    pub sample_time_utc: String,
    pub count: u32,
    pub value: f64,
    pub data_point_count: u32,
    pub includes_secondary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NinjaDetails {
    pub id: u32,
    pub icon: String,
    pub poeTradeId: i32,
    pub name: String,
}
