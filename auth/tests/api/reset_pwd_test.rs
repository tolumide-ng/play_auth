const RESET: &'static str = "/reset";
const INVALID_JWT: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImRzanZkZmhsZ2ZrZmdAZXhhbXBsZS5jb20iLCJ1c2VyX2lkIjoiYjIzZDFkZDctYTQwNS00YjBhLTk5ZDctNWMzOGUxZTZmZTNjIiwidmVyaWZpZWQiOmZhbHNlLCJleHAiOjE2NDc3OTQ4Mzc1NDYsImlhdCI6MTY0Nzc5MzYzNzU0Niwic3ViaiI6IkxvZ2luIn0.tb-ORsut7o5vxdQd_f09O46SDGJTo4bus9TCtiIa7TI";

#[cfg(test)]
mod test {
    use fake::Fake;
    use mockall::predicate::*;
    use rocket::http::ContentType;

    use crate::helpers::app::{get_client, parse_api_response};
    use crate::helpers::app::ResponseType;

    use super::RESET;




    #[rocket::async_test]
    async fn test_invalid_token() {
        let client = get_client().await;
        let response = client.app().put(RESET)
            .header(ContentType::JSON)
            .dispatch();
    }
}