#[derive(Debug)]
pub struct PasswordValidator {}

#[derive(Debug)]
pub struct ErrorList(Vec<String>);

#[derive(Debug)]
enum ReturnValue {
    Ok,
    Err(String),
}

impl PasswordValidator {
    pub fn validate(&self, password: &str) -> (bool, ErrorList) {
        let mut valid = true;
        let mut err_list = ErrorList::new();
        let return_value_list = vec![
            self.validate_length(password), 
            self.validate_2numbers(password),
            self.validate_1capital_letter(password),
            self.validate_1special_char(password),
        ];
        for value in return_value_list {
            match value {
                ReturnValue::Ok => (),
                ReturnValue::Err(s) => {
                    valid = false;
                    err_list.0.push(s);
                },
            }
        }
        
        (valid, err_list)
    }

    fn validate_length(&self, password: &str) -> ReturnValue {
        let password = password.to_owned();
        #[allow(unused_assignments)]
        let mut err = String::new();
        if password.len() < 8 {
            err = String::from("Password must be at least 8 characters");

            return ReturnValue::Err(err)
        }

        ReturnValue::Ok
    }

    fn validate_2numbers(&self, password: &str) -> ReturnValue {
        let password = password.to_owned();
        #[allow(unused_assignments)]
        let mut err = String::new();
        if password.chars().filter(|c| c.is_numeric()).count() < 2 {
            err = String::from("The password must contain at least 2 numbers");

            return ReturnValue::Err(err)
        }

        ReturnValue::Ok
    }

    fn validate_1capital_letter(&self, password: &str) -> ReturnValue {
        let password = password.to_owned();
        #[allow(unused_assignments)]
        let mut err = String::new();
        if password.chars().filter(|c| c.is_uppercase()).count() < 1 {
            err = String::from("password must contain at least one capital letter");

            return ReturnValue::Err(err)
        }

        ReturnValue::Ok
    }

    fn validate_1special_char(&self, password: &str) -> ReturnValue {
        let password = password.to_owned();
        #[allow(unused_assignments)]
        let mut err = String::new();
        if password.chars().filter(|c| c.is_ascii_punctuation()).count() < 1 {
            err = String::from("password must contain at least one special character");

            return ReturnValue::Err(err)
        }

        ReturnValue::Ok
    }
}

impl ErrorList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, err: String) {
        self.0.push(err)
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
        let pass = "A!34567";
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
        let pass = "A!cdefg1";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password with less than 1 number, then return false");
        assert_eq!(errors.len(), 1, "Error list has 1 error");
        let mut errors = errors;
        assert_eq!(errors.pop().unwrap(), String::from("The password must contain at least 2 numbers"));
    }

    #[test]
    fn given_password_with_multiple_errors_then_show_multiple_error_messages() {
        // Arrange
        let pass = "A!bc1";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password with length < 8 and less than 2 numbers, then return false");
        assert_eq!(errors.len(), 2, "there are 2 problems with the password");
        let mut errors = errors;
        assert_eq!(
            errors.pop().unwrap(), 
            String::from("The password must contain at least 2 numbers"),
            "error string: password must contain 2 numbers"
        );
        assert_eq!(
            errors.pop().unwrap(), 
            String::from("Password must be at least 8 characters"),
            "error string: password must be at least 8 chars"
        );
    }

    #[test]
    fn password_with_less_than_1_capital_letter_must_fail() {
        // Arrange
        let pass = "!abcdefg12";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password with less than 1 capital letter, then return false");
        assert_eq!(errors.len(), 1, "Error list has 1 error");
        let mut errors = errors;
        assert_eq!(errors.pop().unwrap(), String::from("password must contain at least one capital letter"));

    }

    #[test]
    fn password_with_less_than_1_special_char_must_fail() {
        // Arrange
        let pass = "Abcdefg12";
        let validator = PasswordValidator {};

        // Act
        let (result, errors) = validator.validate(pass);

        // Assert
        assert_eq!(result, false, "given password with less than 1 special char, then return false");
        assert_eq!(errors.len(), 1, "Error list has 1 error");
        let mut errors = errors;
        assert_eq!(errors.pop().unwrap(), String::from("password must contain at least one special character"));

    }
}