use axum::response::IntoResponse;
// use axum::RequestPartsExt;

// #[axum::async_trait]
// impl<S> axum::extract::FromRequestParts<S> for Cart
// where
//     S: Send + Sync,
// {
//     type Rejection = axum::response::Response;
//     async fn from_request_parts(
//         parts: &mut axum::http::request::Parts,
//         state: &S,
//     ) -> Result<Self, Self::Rejection> {
//         let cookie_jar = axum_extra::extract::cookie::CookieJar::from_request_parts(parts, state)
//             .await
//             .map_err(|err| err.into_response())?;

//         let axum::extract::Extension(db) = parts
//             .extract::<axum::extract::Extension<super::Database>>()
//             .await
//             .map_err(|err| err.into_response())?;

//         let cart_cookie = cookie_jar
//             .get("cart")
//             .and_then(|cookie| serde_json::from_str(&cookie.value()).ok())
//             .unwrap_or(super::CartCookie {
//                 id: format!("cart:{}", ulid::Ulid::new()),
//                 cart_items: vec![],
//             });

//         let cart = super::get_cart(&db, &cart_cookie).await;

//         Ok(Cart(cart))
//     }
// }

pub struct CartCookie(pub Option<crate::cookies::CartCookie>);

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for CartCookie
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookie_jar = axum_extra::extract::cookie::CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;

        let cart_cookie = cookie_jar
            .get("cart")
            .and_then(|cookie| serde_json::from_str(&cookie.value()).ok());

        Ok(CartCookie(cart_cookie))
    }
}
