use crate::products::{domain, repos};

pub async fn add_to_cart(input: AddToCartInput) -> Result<(), AddToCartError> {
    let mut cart = match input.cart_cookie {
        Some(cart_cookie) => {
            let cart_id = cart_cookie
                .id()
                .try_into()
                .map_err(|_| AddToCartError::TBD)?;

            let cart_items = repos::ProductRepo::get_cart_items(cart_cookie.cart_items())
                .await
                .map_err(|_| AddToCartError::TBD)?;

            domain::Cart::new(cart_id, cart_items)
        }
        None => {
            let cart_id = domain::CartId::new(ulid::Ulid::new());

            domain::Cart::new(cart_id, vec![])
        }
    };

    let product_id = (&input.product_id)
        .try_into()
        .map_err(|_| AddToCartError::TBD)?;

    if let Some(cart_item) = cart
        .cart_items_mut()
        .iter_mut()
        .find(|cart_item| cart_item.id() == &product_id)
    {
        cart_item.increment_quantity();
    } else {
        let cart_item = repos::ProductRepo::get_cart_item(&product_id)
            .await
            .map_err(|_| AddToCartError::TBD)?;

        match cart_item {
            None => {
                return Err(AddToCartError::TBD);
            }
            Some(cart_item) => {
                cart.cart_items_mut().push(cart_item);
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct AddToCartInput {
    pub cart_cookie: Option<crate::cookies::CartCookie>,
    pub product_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AddToCartError {
    #[error("")]
    TBD,
}

impl axum::response::IntoResponse for AddToCartError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::TBD => (
                axum::http::StatusCode::OK,
                crate::views::pages::ServerErrorView,
            )
                .into_response(),
        }
    }
}
