#[cfg(test)]
mod tests {
    use lets_expect::*;

    lets_expect! {
        expect(2 + 2) {
            to equal(4)
        }

        expect(2 + 3) {
            to equal(5)
            to not_equal(6)
        }

        expect(2 + 4) {
            to equal_6 {
                equal(6),
                not_equal(5)
            }
        }
    }
}
