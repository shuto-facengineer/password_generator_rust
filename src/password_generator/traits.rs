pub trait PasswordGenerator {
    fn generate_password(&self, length: usize) -> String;
}
