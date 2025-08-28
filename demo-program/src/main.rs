use poem::{Route, Server, get, handler, listener::TcpListener, web::Path};

const FISH: &str = include_str!("../artworks/fish.txt");
const CAT: &str = include_str!("../artworks/cat.txt");
const OCTO: &str = include_str!("../artworks/octo.txt");

#[handler]
fn artwork(Path(name): Path<String>) -> &'static str {
    match name.as_str() {
        "fish" => FISH,
        "cat" => CAT,
        "octo" => OCTO,
        _ => "Doesn't exist",
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/api/art/:name", get(artwork));
    println!("Starting server on port 8080");
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}
