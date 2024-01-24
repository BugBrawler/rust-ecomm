#[derive(Debug)]
pub struct Cart {
    id: super::CartId,
    cart_items: Vec<CartItem>,
}

impl Cart {
    pub fn new(id: super::CartId, cart_items: Vec<CartItem>) -> Self {
        Self { id, cart_items }
    }

    pub fn cart_items_mut(&mut self) -> &mut Vec<CartItem> {
        &mut self.cart_items
    }
}

#[derive(Debug)]
pub struct CartItem {
    id: super::ProductId,
    title: super::ProductTitle,
    slug: super::ProductSlug,
    quantity: u32,
}

impl CartItem {
    pub fn new(
        id: super::ProductId,
        title: super::ProductTitle,
        slug: super::ProductSlug,
        quantity: u32,
    ) -> Self {
        Self {
            id,
            title,
            slug,
            quantity,
        }
    }

    pub fn id(&self) -> &super::ProductId {
        &self.id
    }

    pub fn increment_quantity(&mut self) {
        self.quantity += 1;
    }
}

impl TryFrom<super::super::repos::RawProduct> for CartItem {
    type Error = anyhow::Error;

    fn try_from(product: super::super::repos::RawProduct) -> Result<Self, Self::Error> {
        let ulid = ulid::Ulid::from_string(&product.id.id.to_raw())?;

        Ok(Self {
            id: super::ProductId::new(ulid),
            title: super::ProductTitle::new(product.title)?,
            slug: super::ProductSlug::new(product.slug),
            quantity: 1,
        })
    }
}

impl TryFrom<(super::super::repos::RawProduct, &crate::cookies::CartItem)> for CartItem {
    type Error = anyhow::Error;

    fn try_from(
        (product, cart_item): (super::super::repos::RawProduct, &crate::cookies::CartItem),
    ) -> Result<Self, Self::Error> {
        let ulid = ulid::Ulid::from_string(&product.id.id.to_raw())?;
        let product_id = super::ProductId::new(ulid);

        Ok(super::CartItem::new(
            product_id,
            super::ProductTitle::new(product.title)?,
            super::ProductSlug::new(product.slug),
            *cart_item.quantity(),
        ))
    }
}
