#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(Some(1u8) as Option<u8>) {
            to be_some {
                equal(Some(1)),
                be_some
            }

            to be_some_and equal(1)
        }

        expect(None as Option<String>) {
            to be_none {
                equal(None),
                be_none
            }
        }

    }
}
