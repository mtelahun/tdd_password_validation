# TDD Password Input Validation

## [TDD Manifesto Excercise 3](https://tddmanifesto.com/exercises/)

Create a function that can be used as a validator for the password field of a user registration form. The validation function takes a string as an input and returns a validation result. The validation result should contain a boolean indicating if the password is valid or not, and also a field with the possible validation errors.

### Requirements

1. The password must be at least 8 characters long. If it is not met, then the following error message should be returned: “Password must be at least 8 characters”