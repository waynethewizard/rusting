use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    println!("Serving on localhost 3000");
    let _ = Iron::new(router)
    .http("localhost:3000")
    .expect("could not start iron service");
}

fn response(status: status::Status, msg: String) -> Response {
    let mut r = Response::new();
    r.set_mut(status);
    r.set_mut(msg);
    r
}

fn response_ok(msg: String) -> IronResult<Response> {
    let mut r = response(status::Ok, msg);
    //let mut body: mime::Mime = "text/plain;charset=utf-8".parse().unwrap();
    //println!(body);
    //r.set_mut(body);
    Ok(r)
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    response_ok(
        r#"
        <title>My cool thing</title>
        <h1>What is going on here</h1>
        "#
        .to_string(),
    )
}
