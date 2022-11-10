#[derive(Clone, Debug, PartialEq)]
enum Response {
    UserCreated,
    ValidationFailed(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use lets_expect::lets_expect;

    lets_expect! {
        expect(Response::UserCreated) {
            to be_user_created {
                equal(Response::UserCreated),
                not_equal(Response::ValidationFailed("Username is already taken")),
                match_pattern!(Response::UserCreated)
            }
        }

        expect(Response::ValidationFailed("email")) {
            to match_email {
                match_pattern!(Response::ValidationFailed("email")),
                not_match_pattern!(Response::ValidationFailed("email2"))
            }
        }
    }
}
