#[derive(Default, askama::Template)]
#[template(path = "admin/pages/500.html")]
pub struct ServerErrorView;
