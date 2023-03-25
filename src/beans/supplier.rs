use crate::beans::bean::*;
use std::collections::{HashMap, HashSet};

pub struct CartItem(Bean, i32);
pub type Cart = HashSet<CartItem>;

enum Stock {
    NotInStock,
    InStock(i32),
}

pub type Catalog = HashMap<String, (Bean, Stock)>; //TODO: HashMap?

pub trait BeanSupplier {
    fn order(&self, cart: Cart); //TODO: Return Error (request)
    fn get_catalog(&self) -> Catalog;
}
