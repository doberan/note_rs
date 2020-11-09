use crate::controllers::preludes::Controller;

use async_trait::async_trait;
use std::collections::HashMap;
use actix_web::{error, web, Error, HttpResponse, Result};

pub struct HomeController;

#[async_trait]
impl Controller for HomeController {
	async fn index(
		tmpl: web::Data<tera::Tera>,
		query: web::Query<HashMap<String, String>>,
	) -> Result<HttpResponse, Error> {
		let s = tmpl.render("index.html", &tera::Context::new())
			.map_err(|_| error::ErrorInternalServerError("Template error"))?;
		Ok(HttpResponse::Ok().content_type("text/html").body(s))
	}
}