use crate::{products, views};
use axum::response::IntoResponse;

#[axum::debug_handler]
pub async fn home() -> Result<views::HomeView, crate::products::use_cases::GetProductsError> {
    let products = crate::products::use_cases::get_products()
        .await?
        .map(|product| product.into())
        .collect();

    Ok(views::HomeView { products })
}

#[axum::debug_handler]
pub async fn product_detail(
    axum::extract::Path(input): axum::extract::Path<products::use_cases::GetProductDetailInput>,
) -> Result<views::ProductDetailView, products::use_cases::GetProductDetailError> {
    let product = crate::products::use_cases::get_product_detail(input)
        .await?
        .into();

    Ok(views::ProductDetailView { product })
}

#[axum::debug_handler]
pub async fn list_products(
) -> Result<crate::views::pages::ProductListView, crate::products::use_cases::GetProductsError> {
    let products = crate::products::use_cases::get_products()
        .await?
        .map(|product| product.into())
        .collect();

    Ok(crate::views::pages::ProductListView::new(products))
}

#[axum::debug_handler]
pub async fn create_product_view() -> crate::views::partials::CreateProductFormView {
    crate::views::partials::CreateProductFormView {
        form: crate::views::partials::ProductFormData::default(),
    }
}

#[axum::debug_handler]
pub async fn create_product(
    axum::extract::Form(input): axum::extract::Form<crate::products::use_cases::CreateProductInput>,
) -> Result<axum::response::Response, crate::products::use_cases::CreateProductError> {
    crate::products::use_cases::create_product(input).await?;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "HX-Redirect",
        axum::http::HeaderValue::from_static("/admin/products"),
    );
    Ok((headers, ()).into_response())
}

#[axum::debug_handler]
pub async fn update_product_view(
    axum::extract::Path(input): axum::extract::Path<crate::products::use_cases::GetProductInput>,
) -> Result<
    crate::views::partials::UpdateProductFormView,
    crate::products::use_cases::GetProductError,
> {
    let form_data = crate::products::use_cases::get_product(input).await?.into();

    Ok(crate::views::partials::UpdateProductFormView { form: form_data })
}

#[axum::debug_handler]
pub async fn update_product(
    axum::extract::Path(product_id): axum::extract::Path<String>,
    axum::extract::Form(input): axum::extract::Form<UpdateProductInput>,
) -> Result<axum::response::Response, crate::products::use_cases::UpdateProductError> {
    let input = crate::products::use_cases::UpdateProductInput {
        product_id,
        title: input.title,
        description: input.description,
        price: input.price,
    };
    crate::products::use_cases::update_product(input).await?;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "HX-Redirect",
        axum::http::HeaderValue::from_static("/admin/products"),
    );
    Ok((headers, ()).into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateProductInput {
    title: String,
    description: String,
    price: String,
}

#[axum::debug_handler]
pub async fn delete_product(
    axum::extract::Path(input): axum::extract::Path<crate::products::use_cases::DeleteProductInput>,
) -> Result<axum::response::Response, crate::products::use_cases::DeleteProductError> {
    crate::products::use_cases::delete_product(input).await?;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "HX-Redirect",
        axum::http::HeaderValue::from_static("/admin/products"),
    );
    Ok((headers, ()).into_response())
}

#[axum::debug_handler]
pub async fn add_to_cart(
    crate::extractors::CartCookie(cart_cookie): crate::extractors::CartCookie,
    axum::extract::Form(input): axum::extract::Form<AddToCartInput>,
) -> Result<(), crate::products::use_cases::AddToCartError> {
    let input = crate::products::use_cases::AddToCartInput {
        cart_cookie,
        product_id: input.product_id,
    };

    crate::products::use_cases::add_to_cart(input).await?;

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
pub struct AddToCartInput {
    product_id: String,
}
