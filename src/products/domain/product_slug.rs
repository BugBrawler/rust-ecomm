#[derive(Debug)]
pub struct ProductSlug(String);

impl ProductSlug {
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self(slug::slugify(value.as_ref()))
    }
}

impl AsRef<str> for ProductSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductSlug> for String {
    fn from(product_slug: ProductSlug) -> Self {
        product_slug.0
    }
}
