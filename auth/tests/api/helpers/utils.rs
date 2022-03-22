use fake::{Dummy, Fake};
use rand::Rng;
use rand::seq::SliceRandom;
use uuid::Uuid;

pub struct Pwd;

impl Dummy<Pwd> for &'static str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Pwd, rng: &mut R) -> &'static str {
        const VALID_PWDS: &[&str] = &["Pwd|#89*jdssd", "Anot2143@!jjdsk"];
        VALID_PWDS.choose(rng).unwrap()
    }
}

pub fn get_pwd() -> &'static str {
    let password: &str = Pwd.fake();
    password
}


pub fn get_email() -> String {
    let uuid = Uuid::new_v4().to_string();
    let id = uuid.split("-").collect::<Vec<&str>>()[1];
    // Email addresses coming out from safemail aren't unique enough for the tests and they keed clashing
    let email: String = fake::faker::internet::en::SafeEmail().fake();
    let fragments = email.split("@").collect::<Vec<&str>>();
    let new_email = format!("{}{}@{}", fragments[0], id, fragments[1]);

    new_email
}

pub fn get_invalid_email() -> String {
    let invalid_email: String = fake::faker::internet::en::Username().fake();
    invalid_email
}

pub fn get_invalid_jwt() -> &'static str {
    "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImRzanZkZmhsZ2ZrZmdAZXhhbXBsZS5jb20iLCJ1c2VyX2lkIjoiYjIzZDFkZDctYTQwNS00YjBhLTk5ZDctNWMzOGUxZTZmZTNjIiwidmVyaWZpZWQiOmZhbHNlLCJleHAiOjE2NDc3OTQ4Mzc1NDYsImlhdCI6MTY0Nzc5MzYzNzU0Niwic3ViaiI6IkxvZ2luIn0.tb-ORsut7o5vxdQd_f09O46SDGJTo4bus9TCtiIa7TI"
}