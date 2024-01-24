pub struct ProductRepo;

impl ProductRepo {
    pub async fn get_products(
    ) -> anyhow::Result<impl std::iter::Iterator<Item = super::domain::Product>> {
        let mut response = crate::DB
            .query("SELECT * FROM product ORDER BY created_at DESC LIMIT 50 START 0")
            .await?;
        let products: Vec<RawProduct> = response.take(0)?;

        let products = products
            .into_iter()
            .filter_map(|product| super::domain::Product::try_from(product).ok());

        Ok(products)
    }

    pub async fn get_product_by_id(
        product_id: &super::domain::ProductId,
    ) -> anyhow::Result<Option<super::domain::Product>> {
        let raw_id: surrealdb::opt::RecordId = product_id.into();

        let mut response = crate::DB
            .query("SELECT * FROM product WHERE id = $id")
            .bind(("id", raw_id))
            .await?;
        let product: Option<RawProduct> = response.take(0)?;

        match product {
            None => Ok(None),
            Some(product) => Ok(Some(product.try_into()?)),
        }
    }

    pub async fn get_product_by_slug(
        product_slug: &super::domain::ProductSlug,
    ) -> anyhow::Result<Option<super::domain::Product>> {
        let mut response = crate::DB
            .query("SELECT * FROM product WHERE slug = $slug")
            .bind(("slug", product_slug.as_ref()))
            .await?;
        let product: Option<RawProduct> = response.take(0)?;

        match product {
            None => Ok(None),
            Some(product) => Ok(Some(product.try_into()?)),
        }
    }

    pub async fn get_product_by_title(
        product_title: &super::domain::ProductTitle,
    ) -> anyhow::Result<Option<super::domain::Product>> {
        let mut response = crate::DB
            .query("SELECT * FROM product WHERE title = $title")
            .bind(("title", product_title.as_ref()))
            .await?;
        let product: Option<RawProduct> = response.take(0)?;

        match product {
            None => Ok(None),
            Some(product) => Ok(Some(product.try_into()?)),
        }
    }

    pub async fn exists(product_title: &super::domain::ProductTitle) -> anyhow::Result<bool> {
        let mut response = crate::DB
            .query("SELECT * FROM product WHERE title = $title")
            .bind(("title", product_title.as_ref()))
            .await?;
        let product: Option<RawProduct> = response.take(0)?;

        Ok(product.is_some())
    }

    pub async fn save(product: super::domain::Product) -> anyhow::Result<()> {
        if ProductRepo::exists(&product.title).await? {
            let raw_product: RawProduct = product.into();

            crate::DB.query("UPDATE $id SET title = $title, description = $description, slug = $slug, status = $status, price = $price, created_at = $created_at, updated_at = $updated_at").bind(&raw_product).await?;
        } else {
            let raw_product: RawProduct = product.into();

            crate::DB.query("CREATE product SET id = $id, title = $title, description = $description, slug = $slug, status = $status, price = $price, created_at = $created_at, updated_at = $updated_at").bind(&raw_product).await?;
        }

        Ok(())
    }

    pub async fn delete(product_id: &super::domain::ProductId) -> anyhow::Result<()> {
        let raw_id: surrealdb::opt::RecordId = product_id.into();

        crate::DB.query("DELETE $id").bind(("id", raw_id)).await?;

        Ok(())
    }

    pub async fn get_cart_items(
        cart_items: &Vec<crate::cookies::CartItem>,
    ) -> anyhow::Result<Vec<super::domain::CartItem>> {
        let product_ids: Vec<_> = cart_items.iter().map(|cart_item| cart_item.id()).collect();

        let mut response = crate::DB
            .query("SELECT * FROM product $ids")
            .bind(("ids", product_ids))
            .await?;
        let products: Vec<RawProduct> = response.take(0)?;

        let cart_items = products
            .into_iter()
            .zip(cart_items.into_iter())
            .filter_map(|values| values.try_into().ok())
            .collect();

        Ok(cart_items)
    }

    pub async fn get_cart_item(
        product_id: &super::domain::ProductId,
    ) -> anyhow::Result<Option<super::domain::CartItem>> {
        let raw_id: surrealdb::opt::RecordId = product_id.into();

        let mut response = crate::DB
            .query("SELECT * FROM product WHERE id = $id")
            .bind(("id", raw_id))
            .await?;
        let product: Option<RawProduct> = response.take(0)?;

        match product {
            None => Ok(None),
            Some(product) => Ok(Some(product.try_into()?)),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RawProduct {
    pub id: surrealdb::opt::RecordId,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub status: String,
    pub price: i64,
    pub created_at: surrealdb::sql::Datetime,
    pub updated_at: surrealdb::sql::Datetime,
}

impl From<super::domain::Product> for RawProduct {
    fn from(product: super::domain::Product) -> Self {
        Self {
            id: (&product.id).into(),
            title: product.title.into(),
            description: product.description.into(),
            slug: product.slug.into(),
            status: product.status,
            price: product.price.into(),
            created_at: product.created_at.into(),
            updated_at: product.updated_at.into(),
        }
    }
}
