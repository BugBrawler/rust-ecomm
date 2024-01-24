pub async fn get_product(
    input: GetProductInput,
) -> Result<crate::products::dtos::ProductDTO, GetProductError> {
    let product_id = (&input.product_id)
        .try_into()
        .map_err(|_| GetProductError::NotFoundError)?;

    let product = crate::products::repos::ProductRepo::get_product_by_id(&product_id)
        .await
        .map_err(|err| GetProductError::ServerError(err))?
        .ok_or(GetProductError::NotFoundError)?
        .into();

    Ok(product)
}

#[derive(Debug, serde::Deserialize)]
pub struct GetProductInput {
    pub product_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GetProductError {
    #[error("123")]
    ServerError(anyhow::Error),
    #[error("456")]
    NotFoundError,
}

impl axum::response::IntoResponse for GetProductError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::ServerError(_) => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
            Self::NotFoundError => (
                axum::http::StatusCode::OK,
                crate::views::pages::NotFoundView,
            )
                .into_response(),
        }
    }
}
