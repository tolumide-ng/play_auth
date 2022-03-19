pub trait JwtHelper {
    fn get_user(&self) -> uuid::Uuid;
}