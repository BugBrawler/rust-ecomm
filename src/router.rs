mod db {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Product {
        pub id: surrealdb::opt::RecordId,
        pub name: String,
        pub slug: String,
        pub price: Price,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Price {
        pub currency: String,
        pub amount: i64,
    }
}

#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    slug: String,
    price: String,
}

impl From<db::Product> for Product {
    fn from(product: db::Product) -> Self {
        Product {
            id: product.id.to_string(),
            name: product.name,
            slug: product.slug,
            price: format!(
                "{}",
                rusty_money::Money::from_minor(product.price.amount, rusty_money::iso::EUR)
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cart {
    id: String,
    cart_items: Vec<CartItem>,
    total_amount: i64,
}

#[derive(Debug, Clone)]
struct CartItem {
    id: String,
    name: String,
    slug: String,
    price: i64,
    quantity: u32,
}

#[derive(Debug)]
struct CartTmpl {
    cart_items: Vec<CartItemTmpl>,
    total_tax_amount: String,
    total_amount: String,
}

#[derive(Debug)]
struct CartItemTmpl {
    id: String,
    name: String,
    slug: String,
    price: String,
    quantity: u32,
}

impl From<Cart> for CartTmpl {
    fn from(cart: Cart) -> Self {
        Self {
            cart_items: cart
                .cart_items
                .into_iter()
                .map(|cart_item| cart_item.into())
                .collect(),
            total_tax_amount: format!(
                "{}",
                rusty_money::Money::from_minor(0, rusty_money::iso::EUR)
            ),
            total_amount: format!(
                "{}",
                rusty_money::Money::from_minor(cart.total_amount, rusty_money::iso::EUR)
            ),
        }
    }
}

impl From<CartItem> for CartItemTmpl {
    fn from(cart_item: CartItem) -> Self {
        Self {
            id: cart_item.id,
            name: cart_item.name,
            slug: cart_item.slug,
            price: format!(
                "{}",
                rusty_money::Money::from_minor(cart_item.price, rusty_money::iso::EUR)
            ),
            quantity: cart_item.quantity,
        }
    }
}

type Database = std::sync::Arc<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>>;

pub fn router<S>() -> axum::Router<S> {
    let cart_router = axum::Router::new()
        .route("/", axum::routing::post(add_to_cart))
        .route(
            "/:product_id",
            axum::routing::put(update_cart).delete(remove_from_cart),
        );

    let admin_router = axum::Router::new()
        .route(
            "/products",
            axum::routing::get(product_list).post(create_product),
        )
        .route("/products/create", axum::routing::get(product_create));

    let app = axum::Router::new()
        .route("/", axum::routing::get(home))
        .route("product/:slug", axum::routing::get(product_detail))
        .nest("/cart", cart_router)
        .route(
            "/checkout",
            axum::routing::get(checkout).post(handle_checkout),
        )
        .nest("/admin", admin_router)
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));

    let app = app.fallback(handler_404);

    app
}

mod extractors {
    use axum::response::IntoResponse;
    use axum::RequestPartsExt;

    pub struct Cart(pub super::Cart);

    #[axum::async_trait]
    impl<S> axum::extract::FromRequestParts<S> for Cart
    where
        S: Send + Sync,
    {
        type Rejection = axum::response::Response;
        async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            let cookie_jar =
                axum_extra::extract::cookie::CookieJar::from_request_parts(parts, state)
                    .await
                    .map_err(|err| err.into_response())?;

            let axum::extract::Extension(db) = parts
                .extract::<axum::extract::Extension<super::Database>>()
                .await
                .map_err(|err| err.into_response())?;

            let cart_cookie = cookie_jar
                .get("cart")
                .and_then(|cookie| serde_json::from_str(&cookie.value()).ok())
                .unwrap_or(super::CartCookie {
                    id: format!("cart:{}", ulid::Ulid::new()),
                    cart_items: vec![],
                });

            let cart = super::get_cart(&db, &cart_cookie).await;

            Ok(Cart(cart))
        }
    }

    pub struct CartCookie(pub super::CartCookie);

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
            let cookie_jar =
                axum_extra::extract::cookie::CookieJar::from_request_parts(parts, state)
                    .await
                    .map_err(|err| err.into_response())?;

            let cart_cookie = cookie_jar
                .get("cart")
                .and_then(|cookie| serde_json::from_str(&cookie.value()).ok())
                .unwrap_or(super::CartCookie {
                    id: format!("cart:{}", ulid::Ulid::new()),
                    cart_items: vec![],
                });

            Ok(CartCookie(cart_cookie))
        }
    }
}

async fn handler_404() -> (axum::http::StatusCode, impl axum::response::IntoResponse) {
    (axum::http::StatusCode::NOT_FOUND, "nothing to see here")
}

async fn home(
    axum::extract::State(db): axum::extract::State<Database>,
    // cookie_jar: axum_extra::extract::cookie::CookieJar,
    extractors::Cart(cart): extractors::Cart,
) -> impl axum::response::IntoResponse {
    // let cart_cookie: Option<CartCookie> = cookie_jar
    //     .get("cart")
    //     .and_then(|cookie| serde_json::from_str(&cookie.value()).ok());
    // let cookie_value = match cart_cookie {
    //     None => CartCookie {
    //         id: "test".to_string(),
    //         cart_items: vec![],
    //     },
    //     Some(cookie) => cookie,
    // };
    // let cart = get_cart(&db, &cookie_value).await;
    let mut result = db.query("SELECT * FROM product").await.unwrap();
    let products: Vec<db::Product> = result.take(0).unwrap();
    let products: Vec<Product> = products.into_iter().map(|product| product.into()).collect();
    // let Cart { cart_items: vec![], total_tax_amount: format!("{}", ), total_amount }
    let template = HomeTemplate {
        products,
        cart: cart.into(),
    };
    template
}

async fn product_detail(
    axum::extract::State(db): axum::extract::State<Database>,
    axum::extract::Path(slug): axum::extract::Path<String>,
    extractors::Cart(cart): extractors::Cart,
) -> impl axum::response::IntoResponse {
    // let cart = get_cart(&db, &cookie_value).await;
    let mut result = db
        .query("SELECT * FROM product WHERE slug = $slug")
        .bind(("slug", &slug))
        .await
        .unwrap();
    let product: Option<db::Product> = result.take(0).unwrap();
    if let Some(product) = product {
        let template = ProductDetailTemplate {
            product: product.into(),
            cart: cart.into(),
        };
        template
    } else {
        let products: Vec<Product> = vec![];

        let product = products
            .into_iter()
            .find(|product| product.slug == slug)
            .unwrap();

        let template = ProductDetailTemplate {
            product,
            cart: cart.into(),
        };
        template
    }
}

async fn product_create() -> impl axum::response::IntoResponse {
    let template = ProductCreateTemplate;
    template
}

#[derive(Debug, serde::Deserialize)]
struct CreateProductInput {
    name: String,
    price: String,
}

async fn create_product(
    axum::extract::State(db): axum::extract::State<Database>,
    axum::extract::Form(input): axum::extract::Form<CreateProductInput>,
) -> axum::response::Redirect {
    let price = (input.price.parse::<f64>().unwrap() * 100.0).round() as i64;
    let _result = db
        .query("CREATE product:ulid() SET name = $name, slug = $slug, price = $price")
        .bind(("name", &input.name))
        .bind(("slug", slug::slugify(&input.name)))
        .bind((
            "price",
            db::Price {
                currency: "EUR".to_string(),
                amount: price,
            },
        ))
        .await
        .unwrap();

    axum::response::Redirect::to("/admin/products")
}

async fn product_list() -> impl axum::response::IntoResponse {
    ProductListTemplate
}

#[derive(Debug, serde::Deserialize)]
struct AddToCartInput {
    product_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CartCookie {
    id: String,
    cart_items: Vec<CartCookieItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CartCookieItem {
    id: String,
    quantity: u32,
}

impl From<Cart> for CartCookie {
    fn from(cart: Cart) -> Self {
        Self {
            id: cart.id,
            cart_items: cart
                .cart_items
                .into_iter()
                .map(|cart_item| cart_item.into())
                .collect(),
        }
    }
}

impl From<CartItem> for CartCookieItem {
    fn from(cart_item: CartItem) -> Self {
        Self {
            id: cart_item.id,
            quantity: cart_item.quantity,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct UpdateCartInput {
    operation: Operation,
}

#[derive(Debug, serde::Deserialize)]
enum Operation {
    #[serde(rename = "minus")]
    Minus,
    #[serde(rename = "plus")]
    Plus,
}

async fn get_cart(db: &Database, cookie_value: &CartCookie) -> Cart {
    let mut result = db
        .query("SELECT * FROM product WHERE <string> id INSIDE $ids")
        .bind((
            "ids",
            cookie_value
                .cart_items
                .iter()
                .map(|cart_item| &cart_item.id)
                .collect::<Vec<&String>>(),
        ))
        .await
        .unwrap();

    let products: Vec<db::Product> = result.take(0).unwrap();

    let mut cart_items = vec![];
    let mut total_amount: i64 = 0;
    for product in products.into_iter() {
        if let Some(cart_item) = cookie_value
            .cart_items
            .iter()
            .find(|cart_item| cart_item.id == product.id.to_string())
        {
            total_amount += product.price.amount * cart_item.quantity as i64;
            cart_items.push(CartItem {
                id: product.id.to_string(),
                name: product.name,
                slug: product.slug,
                price: product.price.amount,
                quantity: cart_item.quantity,
            });
        }
    }

    let cart = Cart {
        id: cookie_value.id.clone(),
        cart_items,
        total_amount,
    };
    cart
}

#[axum::debug_handler]
async fn add_to_cart(
    axum::extract::State(db): axum::extract::State<Database>,
    cookie_jar: axum_extra::extract::cookie::CookieJar,
    extractors::Cart(mut cart): extractors::Cart,
    axum::extract::Form(input): axum::extract::Form<AddToCartInput>,
) -> impl axum::response::IntoResponse {
    // std::thread::sleep(std::time::Duration::from_secs(3));

    if let Some(cart_item) = cart
        .cart_items
        .iter_mut()
        .find(|item| item.id == input.product_id)
    {
        cart_item.quantity += 1;
    } else {
        let mut result = db
            .query("SELECT * FROM product WHERE <string> id = $id")
            .bind(("id", &input.product_id))
            .await
            .unwrap();

        let product: Option<db::Product> = result.take(0).unwrap();
        if let Some(product) = product {
            cart.cart_items.push(CartItem {
                id: product.id.to_string(),
                name: product.name,
                slug: product.slug,
                price: product.price.amount,
                quantity: 1,
            });
        } else {
            // return error
        }
    }

    let cart_cookie: CartCookie = cart.clone().into();

    let cart_cookie = axum_extra::extract::cookie::Cookie::build(
        "cart",
        serde_json::to_string(&cart_cookie).unwrap(),
    )
    .path("/")
    .finish();

    (
        cookie_jar.add(cart_cookie),
        CartTemplate { cart: cart.into() },
    )
}

#[axum::debug_handler]
async fn remove_from_cart(
    axum::extract::Path(cart_item_id): axum::extract::Path<String>,
    // axum::extract::State(db): axum::extract::State<Database>,
    cookie_jar: axum_extra::extract::cookie::CookieJar,
    extractors::Cart(mut cart): extractors::Cart,
) -> impl axum::response::IntoResponse {
    cart.cart_items = cart
        .cart_items
        .into_iter()
        .filter(|cart_item| cart_item.id != cart_item_id)
        .collect::<Vec<CartItem>>();

    if cart.cart_items.len() > 0 {
        let cart_cookie: CartCookie = cart.clone().into();
        let cart_cookie = axum_extra::extract::cookie::Cookie::new(
            "cart",
            serde_json::to_string(&cart_cookie).unwrap(),
        );

        (
            cookie_jar.add(cart_cookie),
            CartTemplate { cart: cart.into() },
        )
    } else {
        (
            cookie_jar.remove(
                axum_extra::extract::cookie::Cookie::build("cart", "")
                    .path("/")
                    .finish(),
            ),
            CartTemplate { cart: cart.into() },
        )
    }
}

#[axum::debug_handler]
async fn update_cart(
    axum::extract::Path(cart_item_id): axum::extract::Path<String>,
    // axum::extract::State(db): axum::extract::State<Database>,
    cookie_jar: axum_extra::extract::cookie::CookieJar,
    extractors::Cart(mut cart): extractors::Cart,
    axum::extract::Form(input): axum::extract::Form<UpdateCartInput>,
) -> impl axum::response::IntoResponse {
    if let Some(cart_item) = cart
        .cart_items
        .iter_mut()
        .find(|item| item.id == cart_item_id)
    {
        match &input.operation {
            Operation::Minus => {
                cart_item.quantity -= 1;
            }
            Operation::Plus => {
                cart_item.quantity += 1;
            }
        }
    }

    cart.cart_items = cart
        .cart_items
        .into_iter()
        .filter(|cart_item| cart_item.quantity != 0)
        .collect::<Vec<CartItem>>();

    if cart.cart_items.len() > 0 {
        let cart_cookie: CartCookie = cart.clone().into();
        let cart_cookie = axum_extra::extract::cookie::Cookie::build(
            "cart",
            serde_json::to_string(&cart_cookie).unwrap(),
        )
        .path("/")
        .finish();

        (
            cookie_jar.add(cart_cookie),
            CartTemplate { cart: cart.into() },
        )
    } else {
        (
            cookie_jar.remove(
                axum_extra::extract::cookie::Cookie::build("cart", "")
                    .path("/")
                    .finish(),
            ),
            CartTemplate { cart: cart.into() },
        )
    }
}

#[derive(Debug, serde::Deserialize)]
struct CheckoutInput {
    email: String,
    first_name: String,
    last_name: String,
    address: String,
    zip_code: String,
    city: String,
    phone: String,
}

#[axum::debug_handler]
async fn checkout(
    // axum::extract::State(db): axum::extract::State<Database>,
    extractors::Cart(cart): extractors::Cart,
) -> impl axum::response::IntoResponse {
    // let cart = get_cart(&db, &cookie_value).await;
    CheckoutTemplate { cart: cart.into() }
}

#[derive(Debug, serde::Serialize)]
struct BillingAddress {
    first_name: String,
    last_name: String,
    address: String,
    zip_code: String,
    city: String,
    phone: String,
}

#[axum::debug_handler]
async fn handle_checkout(
    axum::extract::State(db): axum::extract::State<Database>,
    extractors::Cart(cart): extractors::Cart,
    axum::extract::Form(input): axum::extract::Form<CheckoutInput>,
) -> impl axum::response::IntoResponse {
    let billing_address = BillingAddress {
        first_name: input.first_name,
        last_name: input.last_name,
        address: input.address,
        zip_code: input.zip_code,
        city: input.city,
        phone: input.phone,
    };
    let _result = db
        .query("CREATE order:ulid() SET status = $status, email = $email, billing_address = $billing_address, shipping_address = $shipping_address")
        .bind(("status", "PENDING"))
        .bind(("email", &input.email))
        .bind(("billing_address", &billing_address))
        .bind(("shipping_address", &billing_address))
        .await
        .unwrap();
    ThankYouTemplate { cart: cart.into() }
}

#[derive(askama::Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    cart: CartTmpl,
    products: Vec<Product>,
}

#[derive(askama::Template)]
#[template(path = "product-detail.html")]
struct ProductDetailTemplate {
    cart: CartTmpl,
    product: Product,
}

#[derive(askama::Template)]
#[template(path = "product-create.html")]
struct ProductCreateTemplate;

#[derive(askama::Template)]
#[template(path = "product-list.html")]
struct ProductListTemplate;

#[derive(askama::Template)]
#[template(path = "Cart.html")]
struct CartTemplate {
    cart: CartTmpl,
}

#[derive(askama::Template)]
#[template(path = "checkout.html")]
struct CheckoutTemplate {
    cart: CartTmpl,
}

#[derive(askama::Template)]
#[template(path = "ThankYou.html")]
struct ThankYouTemplate {
    cart: CartTmpl,
}
