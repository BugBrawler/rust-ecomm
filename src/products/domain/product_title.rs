#[derive(Debug, PartialEq)]
pub struct ProductTitle(String);

impl ProductTitle {
    pub fn new<T: AsRef<str>>(value: T) -> Result<Self, ProductTitleError> {
        let value = value.as_ref().trim();

        if !validator::validate_length(value, Some(3), None, None) {
            return Err(ProductTitleError::TitleIsTooShort {
                value: value.to_owned(),
                msg: "Title has to be at least 3 characters long".to_string(),
            });
        }

        Ok(Self(value.to_owned()))
    }
}

impl AsRef<str> for ProductTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductTitle> for String {
    fn from(product_title: ProductTitle) -> Self {
        product_title.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProductTitleError {
    #[error("Title has to be at least 3 characters long")]
    TitleIsTooShort { value: String, msg: String },
}

impl From<ProductTitleError> for super::ValidationError {
    fn from(err: ProductTitleError) -> Self {
        match err {
            ProductTitleError::TitleIsTooShort { value, msg } => {
                Self::TitleIsTooShort { value, msg }
            }
        }
    }
}
