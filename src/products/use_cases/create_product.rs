pub async fn create_product(input: CreateProductInput) -> Result<(), CreateProductError> {
    let id = crate::products::domain::ProductId::new(ulid::Ulid::new());

    let title = crate::products::domain::ProductTitle::new(&input.title)
        .map_err(|err| CreateProductError::BadRequest(err.into()))?;

    let description = crate::products::domain::ProductDescription::new(&input.description);

    let price = crate::products::domain::ProductPrice::new(&input.price)
        .map_err(|err| CreateProductError::BadRequest(err.into()))?;

    if crate::products::repos::ProductRepo::exists(&title)
        .await
        .map_err(|err| CreateProductError::ServerError(err))?
    {
        return Err(CreateProductError::ProductAlreadyExists {
            value: input.title,
            msg: "Product title is already in use".to_string(),
        });
    }

    let product = crate::products::domain::Product::new(id, title, description, price);

    crate::products::repos::ProductRepo::save(product)
        .await
        .map_err(|err| CreateProductError::ServerError(err))?;

    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateProductInput {
    pub title: String,
    pub description: String,
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateProductError {
    #[error("456")]
    ServerError(anyhow::Error),
    #[error("")]
    BadRequest(crate::products::domain::ValidationError),
    #[error("{msg:?}")]
    ProductAlreadyExists { value: String, msg: String },
}

impl axum::response::IntoResponse for CreateProductError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::ServerError(_) => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
            Self::BadRequest(value) => {
                let input = match value {
                    crate::products::domain::ValidationError::TitleIsTooShort { value, msg } => {
                        crate::views::partials::FormInput::title(value, msg)
                    }
                    crate::products::domain::ValidationError::PriceHasToBeNumeric {
                        value,
                        msg,
                    } => crate::views::partials::FormInput::price(value, msg),
                };

                (
                    axum::http::StatusCode::OK,
                    crate::views::partials::FormInputView { input },
                )
            }
            .into_response(),
            Self::ProductAlreadyExists { value, msg } => {
                let input = crate::views::partials::FormInput::title(value, msg);
                (
                    axum::http::StatusCode::OK,
                    crate::views::partials::FormInputView { input },
                )
            }
            .into_response(),
        }
    }
}
