use crate::controllers::preludes::Controller;

use async_trait::async_trait;
use std::collections::HashMap;
use actix_web::{error, web, Error, HttpResponse, Result};

pub struct UserController;

#[async_trait]
impl Controller for UserController {
	async fn index(
		tmpl: web::Data<tera::Tera>,
		query: web::Query<HashMap<String, String>>,
	) -> Result<HttpResponse, Error> {
    let name = query.get("name");
    // submitted form
    let mut ctx = tera::Context::new();
    ctx.insert("name", &name.to_owned());
    ctx.insert("list", &vec![1,2,3,4,5,6,7]);
    ctx.insert("text", &"Welcome!".to_owned());
    let s = tmpl.render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
	}
}