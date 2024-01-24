#[derive(Default, askama::Template)]
#[template(path = "admin/pages/404.html")]
pub struct NotFoundView;
