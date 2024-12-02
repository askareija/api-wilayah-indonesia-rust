use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Province {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Regency {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub province_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct District {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub regency_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Village {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub district_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullAdminData {
    pub province_code: String,
    pub province_name: String,
    pub city_code: String,
    pub city_name: String,
    pub region_code: String,
    pub region_name: String,
    pub village_code: String,
    pub village_name: String,
}
