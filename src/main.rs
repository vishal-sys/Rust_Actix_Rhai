use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    Responder,
};

use rhai::Engine;


#[get("/multiply/{numb1}/{numb2}")]
async fn multiply(path: Path<(i64,i64)>) -> impl Responder {

    //Get the numbers from the url path
    let (numb1,numb2) = path.into_inner();

    //create an instance of the rhai engine
    let mut engine = Engine::new();

    //register an API that exposes the numbers  to Rhai 
    engine.register_fn("numb1",move || numb1 );
    engine.register_fn("numb2", move || numb2 );

    //run the script
    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap(); 

    //return the result 
    format!("{result}")
}

#[get("/add/{numb1}/{numb2}")]
async fn add(path: Path<(i64,i64)>) -> impl Responder {
    //get the numbers from the url path 
    let (numb1,numb2) = path.into_inner();

    //create an instance of the rhai engine
    let mut engine = Engine::new();

    //register an API that exposes the numbers to Rhai
    engine.register_fn("numb1",move || numb1 );
    engine.register_fn("numb2",move || numb2 );

    //run the script 
    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    //return the result 
    format!("{result}")
}

#[get("/div/{numb1}/{numb2}")]
async fn div(path: Path<(i64,i64)>) -> impl Responder {
    //get the numbers from the url path 
    let (numb1,numb2) = path.into_inner();

    //create an instance of the rhai engine
    let mut engine = Engine::new();

    //register an API that exposes the numbers to Rhai
    engine.register_fn("numb1",move || numb1 );
    engine.register_fn("numb2",move || numb2 );

    //run the script 
    let result = engine.eval_file::<i64>("src/div.rhai".into()).unwrap();

    //return the Result 
    format!("{result}")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(|| {
        App::new()
        .service(multiply)
        .service(add)
        .service(div)
    })

    .bind(("127.0.0.1",8080))
    .unwrap()
    .run()
    .await
}