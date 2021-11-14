use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub price: i32,
    pub decimal_point: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "currencies"]
pub struct NewCurrency<'a> {
    pub symbol: &'a str,
    pub name: &'a str,
    pub price: i32,
    pub decimal_point: i8,
    pub created_at: chrono::NaiveDateTime,
}