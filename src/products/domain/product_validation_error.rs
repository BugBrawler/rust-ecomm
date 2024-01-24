#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("{msg:?}")]
    TitleIsTooShort { value: String, msg: String },
    #[error("{msg:?}")]
    PriceHasToBeNumeric { value: String, msg: String },
}
