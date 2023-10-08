use crate::password_generator::password_algorithm::PasswordAlgorithm;

pub struct PasswordConfig {
    pub length: usize,
    pub algorithm: PasswordAlgorithm,
}
