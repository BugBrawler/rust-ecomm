#[derive(askama::Template)]
#[template(path = "admin/pages/product_list.html")]
pub struct ProductListView {
    products: Vec<ProductListItem>,
}

impl ProductListView {
    pub fn new(products: Vec<ProductListItem>) -> Self {
        Self { products }
    }
}

#[derive(Debug)]
pub struct ProductListItem {
    id: String,
    title: String,
}

impl From<crate::products::dtos::ProductDTO> for ProductListItem {
    fn from(product: crate::products::dtos::ProductDTO) -> Self {
        Self {
            id: product.id,
            title: product.title,
        }
    }
}
