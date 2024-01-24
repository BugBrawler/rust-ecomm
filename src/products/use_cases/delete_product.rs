pub async fn delete_product(input: DeleteProductInput) -> Result<(), DeleteProductError> {
    let product_id = (&input.product_id)
        .try_into()
        .map_err(|_| DeleteProductError::ProductNotFound)?;

    crate::products::repos::ProductRepo::get_product_by_id(&product_id)
        .await
        .map_err(|err| DeleteProductError::ServerError(err))?
        .ok_or(DeleteProductError::ProductNotFound)?;

    crate::products::repos::ProductRepo::delete(&product_id)
        .await
        .map_err(|err| DeleteProductError::ServerError(err))?;

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteProductInput {
    pub product_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteProductError {
    #[error("456")]
    ServerError(anyhow::Error),
    #[error("")]
    ProductNotFound,
}

impl axum::response::IntoResponse for DeleteProductError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::ServerError(_) => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
            Self::ProductNotFound => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
        }
    }
}
