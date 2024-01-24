const TABLE: &str = "cart";

#[derive(Debug)]
pub struct CartId {
    tb: String,
    id: ulid::Ulid,
}

impl CartId {
    pub fn new(id: ulid::Ulid) -> Self {
        Self {
            tb: TABLE.to_string(),
            id,
        }
    }
}

impl TryFrom<&String> for CartId {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let ulid_string = value.split(':').last().ok_or(anyhow::anyhow!("123"))?;
        let ulid = ulid::Ulid::from_string(&ulid_string)?;
        Ok(CartId::new(ulid))
    }
}
