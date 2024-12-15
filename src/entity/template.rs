use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "room.html")]
pub struct RoomTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate<T: Template> {
    pub child: T,
    pub subtitle: String,
    pub js: String,
    pub css: String,
}