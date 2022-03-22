
#[cfg(test)]
mod pwd_tests {
    use crate::helpers::passwords::pwd::Password;
    use crate::helpers::test_helpers::get_appsettings;

    #[test]
    fn alphabets_only_password_is_invalid() {
        let pwd = "password".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_err());
    }

    #[test]
    fn numbers_only_password_is_invalid() {
        let pwd = "12345".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_err());
    }

    #[test]
    fn alphabets_and_numbers_only_is_invalid() {
        let pwd = "1234password".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_err());
    }

    #[test]
    fn password_with_short_length_is_invalid() {
        let pwd = "ATyp23*".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_err());
    }

    #[test]
    fn password_with_special_characters_only_is_invalid() {
        let pwd = "********#######".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_err());
    }

    #[test]
    fn valid_password() {
        let pwd = "Authentication1234\"".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_ok());
    }
}