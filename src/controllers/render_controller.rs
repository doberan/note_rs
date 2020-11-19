use crate::controllers::preludes::Controller;

use async_trait::async_trait;
use std::collections::HashMap;
use actix_web::{error, web, Error, HttpResponse, Result};

pub struct RenderController;

#[async_trait]
impl Controller for RenderController {
	async fn index(
		tmpl: web::Data<tera::Tera>,
		query: web::Query<HashMap<String, String>>,
	) -> Result<HttpResponse, Error> {
    let name = query.get("html_name");
    let s = tmpl.render(format!("{}.html", name.unwrap()).as_str(), &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
	}
}
