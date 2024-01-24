#[derive(Debug)]
pub struct Product {
    pub id: super::ProductId,
    pub title: super::ProductTitle,
    pub description: super::ProductDescription,
    pub slug: super::ProductSlug,
    pub status: String,
    pub price: super::ProductPrice,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Product {
    pub fn new(
        id: super::ProductId,
        title: super::ProductTitle,
        description: super::ProductDescription,
        price: super::ProductPrice,
    ) -> Self {
        let slug = super::ProductSlug::new(title.as_ref());
        let datetime = chrono::Utc::now();

        Self {
            id,
            title,
            description,
            slug,
            status: "".to_string(),
            price,
            created_at: datetime,
            updated_at: datetime,
        }
    }

    pub fn id(&self) -> &super::ProductId {
        &self.id
    }

    pub fn title(&self) -> &super::ProductTitle {
        &self.title
    }

    pub fn description(&self) -> &super::ProductDescription {
        &self.description
    }

    pub fn price(&self) -> &super::ProductPrice {
        &self.price
    }

    pub fn update_title(&mut self, title: super::ProductTitle) {
        self.title = title;
    }

    pub fn update_description(&mut self, description: super::ProductDescription) {
        self.description = description;
    }

    pub fn update_price(&mut self, price: super::ProductPrice) {
        self.price = price;
    }
}

impl TryFrom<super::super::repos::RawProduct> for Product {
    type Error = anyhow::Error;

    fn try_from(product: super::super::repos::RawProduct) -> Result<Self, Self::Error> {
        let ulid = ulid::Ulid::from_string(&product.id.id.to_raw())?;

        Ok(Self {
            id: super::ProductId::new(ulid),
            title: super::ProductTitle::new(product.title)?,
            description: super::ProductDescription::new(product.description),
            slug: super::ProductSlug::new(product.slug),
            status: product.status,
            price: super::ProductPrice::new((product.price / 100).to_string())?,
            created_at: product.created_at.into(),
            updated_at: product.updated_at.into(),
        })
    }
}
