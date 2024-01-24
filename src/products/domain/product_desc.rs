#[derive(Debug, PartialEq)]
pub struct ProductDescription(String);

impl ProductDescription {
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        let value = value.as_ref().trim();

        Self(value.to_owned())
    }
}

impl AsRef<str> for ProductDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductDescription> for String {
    fn from(product_desc: ProductDescription) -> Self {
        product_desc.0
    }
}
