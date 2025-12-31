use axum::{Form, extract::State, response::Redirect};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::{error::TangoError, http::HTTPState};

pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(state): State<HTTPState>,
    Form(form): Form<LoginForm>,
    cookies: CookieJar,
) -> Result<Redirect, TangoError> {
    return Ok(Redirect::to("/"))
}