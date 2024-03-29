//! # Specific request handlers
//! (favicon, robots, sitemap, page_404).
//!

use actix_files::NamedFile;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder, Result};
use tera::{Context, Tera};

use crate::settings;
pub use request_handlers::*;

// REQUEST HANDLERS
// #################################################################################################
pub mod request_handlers {
    use super::*;
    // Favicon
    // =============================================================================================
    pub async fn favicon_ico(app_state: web::Data<settings::AppState>) -> Result<NamedFile> {
        let icon_path = app_state.format_static_root("favicons/favicon.ico");
        Ok(NamedFile::open(icon_path)?)
    }
    pub async fn favicon(
        app_state: web::Data<settings::AppState>,
        path: web::Path<(String,)>,
    ) -> Result<NamedFile> {
        let path = path.into_inner();
        let icon_path = app_state.format_static_root(format!("favicons/{}", path.0).as_str());
        Ok(NamedFile::open(icon_path)?)
    }
    // Robots
    // =============================================================================================
    pub async fn robots(req: HttpRequest, tmpl: web::Data<Tera>) -> impl Responder {
        let mut ctx = Context::new();
        ctx.insert("scheme", &req.connection_info().scheme().to_owned());
        ctx.insert("host", &req.connection_info().host().to_owned());
        let rendered = tmpl.render("robots.txt", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
    // Page 404
    // =============================================================================================
    pub async fn page_404(app_state: web::Data<settings::AppState>) -> Result<NamedFile> {
        let path = app_state.format_template("404.html");
        Ok(NamedFile::open(path)?.set_status_code(http::StatusCode::NOT_FOUND))
    }
    // Sitemap
    // =============================================================================================
    pub async fn sitemap(app_state: web::Data<settings::AppState>) -> Result<NamedFile> {
        let path = app_state.format_template("sitemap.xml");
        Ok(NamedFile::open(path)?)
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use std::collections::HashMap;

    // Handlers - Unit Tests
    // *********************************************************************************************
    #[actix_rt::test]
    async fn test_handlers_ok() {
        let app_state = web::Data::new(settings::AppState::new());
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .data(tera)
                .route("/favicon.ico", web::get().to(favicon_ico))
                .route("/favicons/{icon}", web::get().to(favicon))
                .route("/robots.txt", web::get().to(robots))
                .route("/sitemap.xml", web::get().to(sitemap))
                .default_service(web::route().to(page_404)),
        )
        .await;

        let mut handlers = HashMap::new();
        handlers.insert("favicon_ico", "/favicon.ico");
        handlers.insert("favicon", "/favicons/favicon.png");
        handlers.insert("robots", "/robots.txt");
        handlers.insert("sitemap", "/sitemap.xml");
        handlers.insert("page_404", "/test-page-404");

        for (handler, route) in &handlers {
            let req = test::TestRequest::get().uri(route).to_request();
            let resp = test::call_service(&mut app, req).await;
            assert_eq!(
                resp.status(),
                match handler {
                    &"page_404" => http::StatusCode::NOT_FOUND,
                    _ => http::StatusCode::OK,
                },
                "Error - Handler: {0} ; Route: `{1}`",
                handler,
                route
            );
        }
    }
}
