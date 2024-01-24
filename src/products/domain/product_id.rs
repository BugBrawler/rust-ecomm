#[derive(Debug, PartialEq)]
pub struct ProductId {
    tb: String,
    id: ulid::Ulid,
}

impl ProductId {
    pub fn new(id: ulid::Ulid) -> Self {
        Self {
            tb: "product".to_string(),
            id,
        }
    }
}

impl From<&ProductId> for surrealdb::opt::RecordId {
    fn from(product_id: &ProductId) -> Self {
        Self::from((product_id.tb.to_owned(), product_id.id.to_string()))
    }
}

impl From<ProductId> for String {
    fn from(product_id: ProductId) -> Self {
        format!("{}:{}", product_id.tb, product_id.id.to_string())
    }
}

impl TryFrom<&String> for ProductId {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let ulid_string = value.split(':').last().ok_or(anyhow::anyhow!("123"))?;
        let ulid = ulid::Ulid::from_string(&ulid_string)?;
        Ok(ProductId::new(ulid))
    }
}
