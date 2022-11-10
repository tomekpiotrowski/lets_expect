struct User {
    name: String,
    password: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AuthenticationError {
    message: String,
}

struct Page {
    pub logged_in: bool,
}

impl Page {
    pub fn new() -> Self {
        Self { logged_in: false }
    }

    pub fn login(&mut self, user: &User) -> Result<(), AuthenticationError> {
        if user.name == "valid_name" && user.password == "valid_password" {
            self.logged_in = true;

            Ok(())
        } else {
            Err(AuthenticationError {
                message: "Invalid credentials".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lets_expect::*;

    lets_expect! {
        let mut page = Page::new();

        let invalid_user = User {
            name: "invalid".to_string(),
            password: "invalid".to_string()
        };
        let valid_user = User {
            name: "valid_name".to_string(),
            password: "valid_password".to_string()
        };

        story login_is_successful {
            expect(page.logged_in) to be_false

            let login_result = page.login(&invalid_user);

            expect(&login_result) to be_err
            expect(&login_result) to equal(Err(AuthenticationError { message: "Invalid credentials".to_string() }))
            expect(page.logged_in) to be_false

            let login_result = page.login(&valid_user);

            expect(login_result) to be_ok
            expect(page.logged_in) to be_true
        }

    }
}
