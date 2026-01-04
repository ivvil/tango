use askama::Template;
use axum::{Form, extract::State, response::{Html, IntoResponse, Redirect}};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::{error::TangoError, http::{HTTPState, webui::templates::login::LoginTemplate}};

pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn login_post(
    State(state): State<HTTPState>,
    Form(form): Form<LoginForm>,
    cookies: CookieJar,
) -> Result<Redirect, TangoError> {
	if form.username == state.config.admin_default_username && form.password == state.config.admin_default_password {
		return Ok(Redirect::to("/"))
	} 

	return Err(TangoError::Unauthorized);
}

pub async fn login() -> Result<impl IntoResponse, TangoError> {
	let tmplt = LoginTemplate{};

	Ok(Html(tmplt.render()?))
}
