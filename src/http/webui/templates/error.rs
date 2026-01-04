use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
	pub shorterror: &'a str,
	pub error: &'a str,
}
