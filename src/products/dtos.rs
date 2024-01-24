#[derive(Debug)]
pub struct ProductDTO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: String,
    pub slug: String,
}

impl From<crate::products::domain::Product> for ProductDTO {
    fn from(product: crate::products::domain::Product) -> Self {
        Self {
            id: product.id.into(),
            title: product.title.into(),
            description: product.description.into(),
            price: product.price.into(),
            slug: product.slug.into(),
        }
    }
}

#[derive(Debug)]
pub struct CartItemDTO {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub quantity: u32,
}

impl From<crate::products::domain::CartItem> for CartItemDTO {
    fn from(cart_item: crate::products::domain::CartItem) -> Self {
        Self {
            id: cart_item.id().into(),
            title: cart_item.title.into(),
            slug: cart_item.slug.into(),
            quantity: cart_item.quantity.into(),
        }
    }
}
