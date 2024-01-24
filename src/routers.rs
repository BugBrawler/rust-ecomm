pub struct Router;

impl Router {
    pub fn build() -> axum::Router<super::AppState> {
        let cart_router =
            axum::Router::new().route("/", axum::routing::post(super::handlers::add_to_cart));

        let admin_router = axum::Router::new()
            .route(
                "/products",
                axum::routing::get(super::handlers::list_products),
            )
            .route(
                "/products/create",
                axum::routing::get(super::handlers::create_product_view)
                    .post(super::handlers::create_product),
            )
            .route(
                "/products/:product_id",
                axum::routing::get(super::handlers::update_product_view)
                    .put(super::handlers::update_product)
                    .delete(super::handlers::delete_product),
            );

        axum::Router::new()
            .route("/", axum::routing::get(super::handlers::home))
            .route(
                "/products/:product_slug",
                axum::routing::get(crate::handlers::product_detail),
            )
            .nest("/cart", cart_router)
            .nest("/admin", admin_router)
            .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
    }
}
