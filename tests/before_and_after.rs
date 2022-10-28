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

        when there_are_two_messages {
            before {
                messages.push("second message");
            }

            after {
                messages.remove(1);
            }

            expect(messages.len()) { to equal(2) }
            expect(messages.get(1).unwrap()) { to equal("second message") }

        }

        story expect_messages_to_not_be_empty {
            expect(messages.len()) to equal(1)

            messages.push("new message");

            expect(&messages) to have(len()) equal(2)
        }
    }
}
