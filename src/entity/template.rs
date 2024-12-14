use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}