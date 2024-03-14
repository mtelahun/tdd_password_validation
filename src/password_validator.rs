#[derive(Debug)]
struct PasswordValidator {}

#[derive(Debug)]
struct ErrorList(Vec<String>);

impl PasswordValidator {
    pub fn validate(&self, password: &str) -> (bool, ErrorList) {
        let password = password.to_owned();
        let mut valid = true;
        let mut err_list = ErrorList::new();

        if password.len() < 8 {
            valid = false;
            err_list.0.push(String::from("Password must be at least 8 characters"));
        } else if password.chars().filter(|c| c.is_numeric()).count() < 2 {
            valid = false;
            err_list.0.push(String::from("The password must contain at least 2 numbers"));
        }
        
        (valid, err_list)
    }
}

impl ErrorList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn pop(&mut self) -> Option<String> {
        self.0.pop()
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
        let mut errors = errors;
        assert_eq!(errors.pop().unwrap(), String::from("Password must be at least 8 characters"));
    }

    #[test]
    fn given_password_when_less_than_2_numbers_then_error() {
        // Arrange
        let pass = "abcdefg1";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password with less than 1 number, then return false");
        assert_eq!(errors.len(), 1, "Error list has 1 error");
        let mut errors = errors;
        assert_eq!(errors.pop().unwrap(), String::from("The password must contain at least 2 numbers"));
    }
}