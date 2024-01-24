#[derive(Debug, PartialEq)]
pub struct ProductPrice {
    currency: &'static Currency,
    amount: i64,
}

impl ProductPrice {
    pub fn new<T: AsRef<str>>(value: T) -> Result<Self, ProductPriceError> {
        let value = value.as_ref().trim();

        let value = value
            .parse::<f64>()
            .map_err(|_| ProductPriceError::PriceHasToBeNumeric {
                value: value.to_owned(),
                msg: "Price has to be numeric".to_string(),
            })?;

        let amount = (value * 100.).round() as i64;

        Ok(Self {
            currency: rusty_money::iso::EUR,
            amount,
        })
    }
}

impl From<ProductPrice> for i64 {
    fn from(product_price: ProductPrice) -> Self {
        product_price.amount
    }
}

impl From<ProductPrice> for String {
    fn from(product_price: ProductPrice) -> Self {
        let money = rusty_money::Money::from_minor(product_price.amount, product_price.currency);

        rusty_money::Formatter::money(&money, rusty_money::Params::default())
    }
}

type Currency = rusty_money::iso::Currency;

#[derive(Debug, thiserror::Error)]
pub enum ProductPriceError {
    #[error("")]
    PriceHasToBeNumeric { value: String, msg: String },
}

impl From<ProductPriceError> for super::ValidationError {
    fn from(err: ProductPriceError) -> Self {
        match err {
            ProductPriceError::PriceHasToBeNumeric { value, msg } => {
                Self::PriceHasToBeNumeric { value, msg }
            }
        }
    }
}
