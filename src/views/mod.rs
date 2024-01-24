pub mod pages;
pub mod partials;

use crate::products;

#[derive(askama::Template)]
#[template(path = "home.html")]
pub struct HomeView {
    pub products: Vec<ProductListing>,
}

#[derive(askama::Template)]
#[template(path = "product_detail.html")]
pub struct ProductDetailView {
    pub product: ProductDetail,
}

#[derive(askama::Template)]
#[template(path = "cart.html")]
pub struct CartView {
    pub cart: Cart,
}

#[derive(Debug)]
pub struct ProductListing {
    id: String,
    title: String,
    slug: String,
}

impl From<products::dtos::ProductDTO> for ProductListing {
    fn from(product: products::dtos::ProductDTO) -> Self {
        Self {
            id: product.id,
            title: product.title,
            slug: product.slug,
        }
    }
}

#[derive(Debug)]
pub struct ProductDetail {
    id: String,
    title: String,
}

impl From<products::dtos::ProductDTO> for ProductDetail {
    fn from(product: products::dtos::ProductDTO) -> Self {
        Self {
            id: product.id,
            title: product.title,
        }
    }
}

#[derive(Debug)]
pub struct Cart {
    cart_items: Vec<CartItem>,
}

#[derive(Debug)]
pub struct CartItem {
    id: String,
    title: String,
    slug: String,
    quantity: u32,
}

// impl From<products::domain::CartItem> for CartItem {
//     fn from(cart: products::domain::CartItem) -> Self {
//         Self { id: cart.id(), title: , slug: , quantity:  }
//     }
// }
