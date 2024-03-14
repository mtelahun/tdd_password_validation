#[derive(Debug)]
struct PasswordValidator {}

#[derive(Debug)]
struct ErrorList(Vec<String>);

impl PasswordValidator {
    pub fn validate(&self, password: &str) -> (bool, ErrorList) {
        let mut err_list = ErrorList::new();
        err_list.0.push(String::from("Password must be at least 8 characters"));
        
        (false, err_list)
    }
}

impl ErrorList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::password_validator::PasswordValidator;

    #[test]
    fn given_password_when_length_less_than_8_chars_then_error() {
        // Arrange
        let pass = "1234567";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password length 7, then return false");
        assert_eq!(errors.len(), 1, "Error list has 1 error");
    }
}