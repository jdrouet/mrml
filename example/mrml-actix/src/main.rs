use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use mrml;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RenderPayload {
    template: String,
}

#[post("/render")]
async fn render(params: web::Form<RenderPayload>) -> impl Responder {
    match mrml::to_html(params.template.as_str(), mrml::Options::default()) {
        Ok(value) => HttpResponse::Ok().content_type("text/plain").body(value),
        Err(err) => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("error: {:?}", err)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(render)
    })
    .bind(get_address())?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::{render, RenderPayload};
    use actix_web::http::StatusCode;
    use actix_web::{middleware, test, App};

    #[actix_rt::test]
    async fn test_render() {
        let mut app = test::init_service(
            App::new()
                .wrap(middleware::Logger::default())
                .service(render),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/render")
            .set_form(&RenderPayload {
                template: r#"
                <mjml>
                    <mj-body>
                        <mj-section>
                            <mj-column>
                                <mj-text>Hello world!</mj-text>
                            </mj-column>
                        </mj-section>
                    </mj-body>
                </mjml>
                "#
                .to_string(),
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let result = test::read_body(resp).await;
        let result = std::str::from_utf8(result.as_ref()).unwrap();
        assert_eq!(result.contains("Hello world!"), true);
    }
}
