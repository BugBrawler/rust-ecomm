pub async fn get_products(
) -> Result<impl std::iter::Iterator<Item = crate::products::dtos::ProductDTO>, GetProductsError> {
    let products = super::super::repos::ProductRepo::get_products()
        .await
        .map_err(|err| GetProductsError::ServerError(err))?
        .map(|product| product.into());

    Ok(products)
}

// pub struct GetProductsInput {
//     offset:
// }

#[derive(Debug)]
pub enum GetProductsError {
    ServerError(anyhow::Error),
}

impl axum::response::IntoResponse for GetProductsError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::ServerError(_) => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
        }
    }
}
