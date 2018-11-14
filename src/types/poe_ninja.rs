//! This module is for the types that end up getting deserialized from
//! Poe.ninja. They're not necessarily the types used here, but needed as
//! deserialization target.

#[derive(Serialize, Deserialize, Debug)]
pub struct NinjaCurrencyOverviewResponse {
    pub lines: Vec<NinjaLineResponse>,
    pub currencyDetails: Vec<NinjaDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NinjaLineResponse {
    pub currencyTypeName: String,
    pub pay: Option<NinjaPrice>,
    pub receive: Option<NinjaPrice>,
    pub chaosEquivalent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
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
pub struct NinjaDetails {
    pub id: u32,
    pub icon: String,
    pub poeTradeId: i32,
    pub name: String
}
