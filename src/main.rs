#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Product {
    id: surrealdb::opt::RecordId,
    name: String,
    slug: String,
}

type Database = surrealdb::Surreal<surrealdb::engine::remote::ws::Client>;

#[tokio::main]
async fn main() {
    let db = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("localhost:8000")
        .await
        .unwrap();

    db.use_ns("test").use_db("test").await.unwrap();

    let app = axum::Router::new()
        .route("/", axum::routing::get(home))
        .route("/product/:slug", axum::routing::get(product_detail))
        .route(
            "/admin/products",
            axum::routing::get(product_list).post(create_product),
        )
        .route("/admin/products/create", axum::routing::get(product_create))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:4321".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home(
    axum::extract::State(db): axum::extract::State<Database>,
) -> impl axum::response::IntoResponse {
    let mut result = db.query("SELECT * FROM product").await.unwrap();
    let products: Vec<Product> = result.take(0).unwrap();
    let template = HomeTemplate { products };
    HtmlTemplate(template)
}

async fn product_detail(
    axum::extract::State(db): axum::extract::State<Database>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let mut result = db
        .query("SELECT * FROM product WHERE slug = $slug")
        .bind(("slug", &slug))
        .await
        .unwrap();
    let product: Option<Product> = result.take(0).unwrap();
    if let Some(product) = product {
        let template = ProductDetailTemplate {
            product: product.clone(),
        };
        HtmlTemplate(template)
    } else {
        let products: Vec<Product> = vec![];

        let product = products
            .iter()
            .find(|product| product.slug == slug)
            .unwrap();

        let template = ProductDetailTemplate {
            product: product.clone(),
        };
        HtmlTemplate(template)
    }
}

async fn product_create() -> impl axum::response::IntoResponse {
    let template = ProductCreateTemplate;
    HtmlTemplate(template)
}

#[derive(Debug, serde::Deserialize)]
struct CreateProductInput {
    name: String,
}

async fn create_product(
    axum::extract::State(db): axum::extract::State<
        surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    >,
    axum::extract::Form(input): axum::extract::Form<CreateProductInput>,
) -> axum::response::Redirect {
    let _result = db
        .query("CREATE product:ulid() SET name = $name, slug = $slug")
        .bind(("name", &input.name))
        .bind(("slug", slug::slugify(&input.name)))
        .await
        .unwrap();

    axum::response::Redirect::to("/admin/products")
}

async fn product_list() -> impl axum::response::IntoResponse {
    let template = ProductListTemplate;
    HtmlTemplate(template)
}

#[derive(askama::Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    products: Vec<Product>,
}

#[derive(askama::Template)]
#[template(path = "product-detail.html")]
struct ProductDetailTemplate {
    product: Product,
}

#[derive(askama::Template)]
#[template(path = "product-create.html")]
struct ProductCreateTemplate;

#[derive(askama::Template)]
#[template(path = "product-list.html")]
struct ProductListTemplate;

struct HtmlTemplate<T>(T);

impl<T> axum::response::IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
