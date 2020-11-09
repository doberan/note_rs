use async_trait::async_trait;

use std::collections::HashMap;

use actix_web::{web, Error, HttpResponse, Result};

macro_rules! get {
    ($p:expr,$f:expr) => {
        web::resource($p).route(web::get().to($f))
    };
}

#[async_trait]
pub trait Controller {
	async fn index(
		tmpl: web::Data<tera::Tera>,
		query: web::Query<HashMap<String, String>>,
	) -> Result<HttpResponse, Error>;
}