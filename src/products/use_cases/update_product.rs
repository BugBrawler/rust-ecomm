pub async fn update_product(input: UpdateProductInput) -> Result<(), UpdateProductError> {
    let title = crate::products::domain::ProductTitle::new(&input.title)
        .map_err(|err| UpdateProductError::BadRequest(err.into()))?;

    let description = crate::products::domain::ProductDescription::new(&input.description);

    let price = crate::products::domain::ProductPrice::new(&input.price)
        .map_err(|err| UpdateProductError::BadRequest(err.into()))?;

    let product_id = (&input.product_id)
        .try_into()
        .map_err(|_| UpdateProductError::ProductNotFound)?;

    let mut product = crate::products::repos::ProductRepo::get_product_by_id(&product_id)
        .await
        .map_err(|err| UpdateProductError::ServerError(err))?
        .ok_or(UpdateProductError::ProductNotFound)?;

    if let Some(existing_product) =
        crate::products::repos::ProductRepo::get_product_by_title(&title)
            .await
            .map_err(|err| UpdateProductError::ServerError(err))?
    {
        if product.id() != existing_product.id() {
            return Err(UpdateProductError::ProductAlreadyExists {
                value: input.title,
                msg: "Product title is already in use".to_string(),
            });
        }
    }

    if &title != product.title() {
        product.update_title(title);
    }

    if &description != product.description() {
        product.update_description(description);
    }

    if &price != product.price() {
        product.update_price(price);
    }

    crate::products::repos::ProductRepo::save(product)
        .await
        .map_err(|err| UpdateProductError::ServerError(err))?;

    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpdateProductInput {
    pub product_id: String,
    pub title: String,
    pub description: String,
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateProductError {
    #[error("456")]
    ServerError(anyhow::Error),
    #[error("")]
    BadRequest(crate::products::domain::ValidationError),
    #[error("")]
    ProductNotFound,
    #[error("{msg:?}")]
    ProductAlreadyExists { value: String, msg: String },
}

impl axum::response::IntoResponse for UpdateProductError {
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
            Self::ProductNotFound => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
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
