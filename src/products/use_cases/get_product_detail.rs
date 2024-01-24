pub async fn get_product_detail(
    input: GetProductDetailInput,
) -> Result<crate::products::dtos::ProductDTO, GetProductDetailError> {
    let product_slug = crate::products::domain::ProductSlug::new(&input.product_slug);

    let product = crate::products::repos::ProductRepo::get_product_by_slug(&product_slug)
        .await
        .map_err(|err| GetProductDetailError::ServerError(err))?
        .ok_or(GetProductDetailError::NotFoundError)?
        .into();

    Ok(product)
}

#[derive(Debug, serde::Deserialize)]
pub struct GetProductDetailInput {
    pub product_slug: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GetProductDetailError {
    #[error("123")]
    ServerError(anyhow::Error),
    #[error("456")]
    NotFoundError,
}

impl axum::response::IntoResponse for GetProductDetailError {
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
