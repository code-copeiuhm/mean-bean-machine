use crate::beans::bean::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct CartItem(Bean, i32);
pub type Cart = HashSet<CartItem>;

#[derive(Debug, Clone, Deserialize, Serialize)]
enum StockStatus {
    NotInStock,
    InStock(i32),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CatalogItem {
    bean: Bean,
    price: f32,
    stock_status: StockStatus,
}

pub type Catalog = HashMap<String, CatalogItem>; //TODO: HashMap?

pub trait BeanSupplier {
    fn order(&self, cart: Cart); //TODO: Return Error (request)
    fn get_catalog(&self) -> Catalog;
    fn get_name(&self) -> String;
    fn get_address(&self) -> String; //TODO: String? IP-addr, or real?
}
