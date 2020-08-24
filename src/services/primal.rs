use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

use super::super::settings;

// CONFIGURE URLs ==================================================================================
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}

// REQUEST HANDLERS ================================================================================
// Home page
async fn index(app_state: web::Data<settings::AppState>, tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", &app_state.get_app_name().to_owned());
    ctx.insert(
        "description",
        &"Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_owned(),
    );
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};

    // Handlers ------------------------------------------------------------------------------------
    #[actix_rt::test]
    async fn test_index_ok() {
        let app_state = web::Data::new(settings::AppState::new());
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .data(tera)
                .route("/", web::get().to(index)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
