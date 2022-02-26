
#[get("/")]
pub fn health() -> &'static str {
    "Hello World from tolumide"
}