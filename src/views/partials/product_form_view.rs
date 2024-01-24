#[derive(askama::Template)]
#[template(path = "admin/pages/create_product.html")]
pub struct CreateProductFormView {
    pub form: ProductFormData,
}

#[derive(askama::Template)]
#[template(path = "admin/pages/edit_product.html")]
pub struct UpdateProductFormView {
    pub form: ProductFormData,
}

#[derive(askama::Template)]
#[template(path = "admin/partials/product_form.html")]
pub struct ProductFormView {
    pub form: ProductFormData,
}

#[derive(askama::Template)]
#[template(path = "admin/partials/text_input.html")]
pub struct FormInputView {
    pub input: FormInput,
}

#[derive(Debug)]
pub struct ProductFormData {
    pub variant: FormVariant,
    pub inputs: Vec<FormInput>,
}

impl Default for ProductFormData {
    fn default() -> Self {
        Self {
            variant: FormVariant::Create,
            inputs: vec![
                FormInput {
                    r#type: InputType::TextInput,
                    name: "title".to_string(),
                    label: "Product title".to_string(),
                    value: "".to_string(),
                    error: "".to_string(),
                    swap_id: "prod_title".to_string(),
                },
                FormInput {
                    r#type: InputType::RichTextInput,
                    name: "description".to_string(),
                    label: "Product description".to_string(),
                    value: "".to_string(),
                    error: "".to_string(),
                    swap_id: "prod_desc".to_string(),
                },
                FormInput {
                    r#type: InputType::TextInput,
                    name: "price".to_string(),
                    label: "Product price".to_string(),
                    value: "".to_string(),
                    error: "".to_string(),
                    swap_id: "prod_price".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct FormInput {
    pub r#type: InputType,
    pub name: String,
    pub label: String,
    pub value: String,
    pub error: String,
    pub swap_id: String,
}

#[derive(Debug)]
pub enum InputType {
    TextInput,
    RichTextInput,
}

impl FormInput {
    pub fn title(value: String, error: String) -> Self {
        Self {
            r#type: InputType::TextInput,
            name: "title".to_string(),
            label: "Product title".to_string(),
            value,
            error,
            swap_id: "prod_title".to_string(),
        }
    }

    // pub fn description(value: String, error: String) -> Self {
    //     Self {
    //         id: "description".to_string(),
    //         name: "description".to_string(),
    //         label: "Product description".to_string(),
    //         value,
    //         error,
    //         swap_id: "prod_desc".to_string(),
    //     }
    // }

    pub fn price(value: String, error: String) -> Self {
        Self {
            r#type: InputType::TextInput,
            name: "price".to_string(),
            label: "Product price".to_string(),
            value,
            error,
            swap_id: "prod_price".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub enum FormVariant {
    #[default]
    Create,
    Update(String),
}

// impl ProductFormData {
//     pub fn new(title: String, description: String, price: String) -> Self {
//         Self {

//             title: FormInput {
//                 value: title,
//                 error: "".to_string(),
//             },
//             description: FormInput {
//                 value: description,
//                 error: "".to_string(),
//             },
//             price: FormInput {
//                 value: price,
//                 error: "".to_string(),
//             },
//         }
//     }

//     pub fn errors(mut self, errs: Vec<crate::products::domain::ValidationError>) -> Self {
//         for err in errs {
//             match err {
//                 crate::products::domain::ValidationError::TitleIsTooShort => {
//                     self.title.error = "Title has to be at least 3 characters long".to_string();
//                 }
//                 crate::products::domain::ValidationError::PriceHasToBeNumeric => {
//                     self.price.error = "Price has to be numeric".to_string();
//                 }
//             }
//         }
//         self
//     }
// }

impl From<crate::products::dtos::ProductDTO> for ProductFormData {
    fn from(product: crate::products::dtos::ProductDTO) -> Self {
        // Self::new(product.title, product.description, product.price)
        Self {
            variant: FormVariant::Update(product.id),
            inputs: vec![
                FormInput {
                    r#type: InputType::TextInput,
                    name: "title".to_string(),
                    label: "Product title".to_string(),
                    value: product.title,
                    error: "".to_string(),
                    swap_id: "prod_title".to_string(),
                },
                FormInput {
                    r#type: InputType::RichTextInput,
                    name: "description".to_string(),
                    label: "Product description".to_string(),
                    value: product.description,
                    error: "".to_string(),
                    swap_id: "prod_desc".to_string(),
                },
                FormInput {
                    r#type: InputType::TextInput,
                    name: "price".to_string(),
                    label: "Product price".to_string(),
                    value: product.price,
                    error: "".to_string(),
                    swap_id: "prod_price".to_string(),
                },
            ],
        }
    }
}

// impl From<crate::products::use_cases::CreateProductInput> for ProductFormData {
//     fn from(input: crate::products::use_cases::CreateProductInput) -> Self {
//         Self::new(input.title, input.description, input.price)
//     }
// }

// impl From<crate::products::use_cases::UpdateProductInput> for ProductFormData {
//     fn from(input: crate::products::use_cases::UpdateProductInput) -> Self {
//         Self::new(input.title, input.description, input.price)
//     }
// }

#[derive(Debug, Default)]
pub struct Description {
    pub value: String,
    pub error: String,
}
