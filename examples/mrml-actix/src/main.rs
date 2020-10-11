use actix_multipart::{Field, Multipart};
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use bytes::buf::{Buf, BufMut};
use futures::TryStreamExt;
use mrml;
use std::env;

fn get_address() -> String {
    let hostname = match env::var("ADDRESS") {
        Ok(value) => value,
        Err(_) => "localhost".into(),
    };
    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_) => "3000".into(),
    };
    format!("{}:{}", hostname, port)
}

async fn get_template_content(mut field: Field) -> Option<String> {
    let mut bytes = web::BytesMut::new();
    while let Ok(Some(field)) = field.try_next().await {
        bytes.put(field);
    }
    String::from_utf8(bytes.to_bytes().to_vec()).ok()
}

async fn get_template(mut payload: Multipart) -> Option<String> {
    while let Ok(Some(field)) = payload.try_next().await {
        let content_dispo = field.content_disposition().unwrap();
        let name = content_dispo.get_name().unwrap();
        if name == "template" {
            return get_template_content(field).await;
        }
    }
    None
}

#[post("/render")]
async fn render(payload: Multipart) -> impl Responder {
    let template: String = match get_template(payload).await {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("template missing"),
    };
    match mrml::to_html(template.as_str(), mrml::Options::default()) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::BadRequest().body(format!("error: {:?}", err)),
    }
}

macro_rules! create_app {
    () => {
        App::new()
            .wrap(middleware::Logger::default())
            .service(render)
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| create_app!())
        .bind(get_address())?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::{Method, StatusCode};
    use actix_web::web::BytesMut;
    use actix_web::{middleware, test, App};
    use bytes::buf::BufMut;
    use common_multipart_rfc7578 as cmultipart;

    #[actix_rt::test]
    async fn success() {
        let mut form = cmultipart::client::multipart::Form::default();
        form.add_text("template", "<mjml><mj-body></mj-body></mjml>");
        let content_type = form.content_type();
        let mut body = cmultipart::client::multipart::Body::from(form);
        let mut bytes = BytesMut::new();
        while let Ok(Some(field)) = body.try_next().await {
            bytes.put(field);
        }
        let mut app = test::init_service(create_app!()).await;
        let req = test::TestRequest::with_header("content-type", content_type)
            .method(Method::POST)
            .uri("/render")
            .set_payload(bytes)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
