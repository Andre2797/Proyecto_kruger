
extern crate http;

use http::{Request, Response};
use actix_web::{post,App, Error, HttpResponse, HttpServer, Result, client::ClientResponse, http::StatusCode, web};
use serde::{Deserialize};
mod logica;

//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}
async fn home() -> Result<HttpResponse, Error>{
    
    Ok(
        HttpResponse::build(StatusCode::OK)
        .content_type("text/html;charset=utf-8")
        .body(include_str!("../template/home.html"))
    )
    
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .route("/home", web::get().to(home))
        .service(
            //web::resource("/home/num")
              //.route(web::post().to(showit))
              showit
             
          )
       
                
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}

#[post("/home/num")]
async fn showit(form: web::Form<FormData>)-> HttpResponse{
 
 let mut  respuesta=logica::num_letras(form.numero.to_string());   
  let res=format!("SU respuesta es {:?}",respuesta);
 //Ok( 
      // format!("Welcome {}!", form.numero)
   //   format!("{:?}",respuesta)
      //HttpResponse::build(StatusCode::OK).body(include_str!("../template/respuesta.html")
    //)
    HttpResponse::Ok()
    .body(res)
    

}

#[derive(Deserialize,Debug)]
struct FormData {
    numero: String,
}