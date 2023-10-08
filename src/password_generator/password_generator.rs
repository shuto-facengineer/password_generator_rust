use super::password_algorithm::PasswordAlgorithm;
use super::simple_password_generator::SimplePasswordGenerator;
use super::numeric_password_generator::NumericPasswordGenerator;

pub trait PasswordGenerator {
    fn generate_password(&self, length: usize) -> String;
}

pub fn new_password_generator(algorithm: PasswordAlgorithm) -> Box<dyn PasswordGenerator> {
    match algorithm {
        PasswordAlgorithm::Simple => Box::new(SimplePasswordGenerator::new()),
        PasswordAlgorithm::Numeric => Box::new(NumericPasswordGenerator::new()),
    }
}
