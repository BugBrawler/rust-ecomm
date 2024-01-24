#[derive(Debug, serde::Deserialize)]
pub struct CartCookie {
    id: String,
    cart_items: Vec<CartItem>,
}

impl CartCookie {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn cart_items(&self) -> &Vec<CartItem> {
        &self.cart_items
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CartItem {
    id: String,
    quantity: u32,
}

impl CartItem {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
}
