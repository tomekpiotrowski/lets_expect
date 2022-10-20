#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        let mut messages: Vec<&str> = Vec::new();

        before {
            messages.push("first message");
        }

        after {
            messages.clear();
        }

        expect(messages.len()) { to equal(1) }
        expect(messages.push("new message")) {
            to change(messages.len()) { from(1), to(2) }
        }

        story expect_messages_to_not_be_empty {
            expect(messages.len()) to equal(1)

            messages.push("new message");

            expect(&messages) to have(len()) equal(2)
        }
    }
}
