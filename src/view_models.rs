use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize,)]
pub struct FieldError{
    valid:bool,
    error_message:String,
}

#[derive(Serialize, Deserialize,)]
pub struct RegisterCheck{
    pub username:FieldError,
    pub email:FieldError,
    pub password:FieldError,
    pub confirmpassword:FieldError
}

impl RegisterCheck {
    pub fn new()->RegisterCheck {
        return RegisterCheck { username: FieldError{valid:true,error_message:"".to_string()}, email: FieldError{valid:true,error_message:"".to_string()}, password: FieldError{valid:true,error_message:"".to_string()}, confirmpassword: FieldError{valid:true,error_message:"".to_string()} };
    }
    pub fn has_error(&self)->bool {
        if !self.username.valid || !self.email.valid || !self.password.valid || !self.confirmpassword.valid {
            return true;
        }else {
            return false
        }
    }

    pub fn set_username_error(&mut self,err_message:&str) {
        self.username.error_message = err_message.to_string();
        self.username.valid = false;
    }

    pub fn set_email_error(&mut self,err_message:&str) {
        self.email.error_message = err_message.to_string();
        self.email.valid = false;
    }
}

#[derive(Serialize, Deserialize,)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmpassword: String,
}

impl Register {

    

    pub fn new()->Register {
        return Register { username: "".to_string(), email: "".to_string(), password: "".to_string(), confirmpassword: "".to_string() };
    }

    pub fn validate_email(&self) ->bool {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        return  email_regex.is_match(&self.email);
    }

    pub fn set_error(&self)->RegisterCheck {
        let mut ret_val = RegisterCheck::new();
        if self.username.is_empty(){
            ret_val.username.valid = false;
            ret_val.username.error_message = "Username required".to_string();
        } else if self.username.chars().count() < 4 || self.username.chars().count() > 20 {
            ret_val.username.valid = false;
            ret_val.username.error_message = "Username should be atleast 4 characters and less than 20".to_string();
        }

        if self.email.is_empty(){
            ret_val.email.valid = false;
            ret_val.email.error_message = "email required".to_string();
        } else if ! self.validate_email() {
            ret_val.email.valid = false;
            ret_val.email.error_message = "Email format is invalid".to_string();
        }

        if self.password.is_empty(){
            ret_val.password.valid = false;
            ret_val.password.error_message = "password required".to_string();
        } else if self.password.chars().count() < 4 || self.password.chars().count() > 20 {
            ret_val.password.valid = false;
            ret_val.password.error_message = "password should be atleast 4 characters and less than 20. ".to_string();
        }

        if self.confirmpassword.is_empty(){
            ret_val.confirmpassword.valid = false;
            ret_val.confirmpassword.error_message = "confirmpassword required".to_string();
        } else if self.password.chars().count() < 4 || self.password.chars().count() > 20 {
            ret_val.confirmpassword.valid = false;
            ret_val.confirmpassword.error_message = "confirmpassword should be atleast 4 characters and less than 20. ".to_string();
        }

        if self.confirmpassword != self.password {
            ret_val.confirmpassword.valid = false;
            ret_val.password.valid = false;
            ret_val.password.error_message =format!("{}{}", ret_val.password.error_message, "passwords must match. ");
            ret_val.confirmpassword.error_message =format!("{}{}", ret_val.confirmpassword.error_message, " passwords must match");
        }

        return ret_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rg = RegisterCheck::new();
        assert_eq!(rg.email.valid, true);
        assert_eq!(rg.password.valid, true);
        assert_eq!(rg.confirmpassword.valid, true);
        assert_eq!(rg.username.valid, true);
    }

    #[test]
    fn test_valid() {
        let mut form = Register::new();
        form.password = "ab".to_string();
        form.confirmpassword = "1234567".to_string();
        let c = form.set_error();
        println!("{}",c.password.error_message);
        assert_eq!(c.password.error_message, "password should be atleast 4 characters and less than 20. passwords must match. ".to_string());
    }

    #[test]
    fn test_username() {
        let mut form = Register::new();
        form.username = "ab".to_string();
        let c = form.set_error();
        println!("{}",c.username.error_message);
    }

 
}