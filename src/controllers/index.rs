use rocket::http::ContentType;

#[get("/")]
pub async fn home() -> (ContentType, &'static str) {
    //! Empty home page
    (ContentType::HTML, "<!-- api.analyze.rs --//>")
}
