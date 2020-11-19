use std::collections::HashMap;
use actix_web::{web, Error, HttpResponse, Result};

/// getメソッド用マクロ
#[allow(unused_macros)]
macro_rules! get {
    ($path:expr,$callback:expr) => {
        web::resource($path).route(web::get().to($callback))
    };
}

/// postメソッド用マクロ
#[allow(unused_macros)]
macro_rules! post {
	($path:expr,$callback:expr) => {
			web::resource($path).route(web::post().to($callback))
	};
}

/// putメソッド用マクロ
#[allow(unused_macros)]
macro_rules! put {
	($path:expr,$callback:expr) => {
			web::resource($path).route(web::put().to($callback))
	};
}

/// deleteメソッド用マクロ
#[allow(unused_macros)]
macro_rules! delete {
	($path:expr,$callback:expr) => {
			web::resource($path).route(web::delete().to($callback))
	};
}


/// scope用マクロ
#[allow(unused_macros)]
macro_rules! scope {
	($p:expr,$e:expr) => {
		web::scope($p).wrap($e)
	};
}

use async_trait::async_trait;
/// Controller用トレイト
#[async_trait]
pub trait Controller {
	async fn index(
		tmpl: web::Data<tera::Tera>,
		query: web::Query<HashMap<String, String>>,
	) -> Result<HttpResponse, Error>;
}