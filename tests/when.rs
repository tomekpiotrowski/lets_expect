#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(a + b + c) {
            when(a = 2) {
                when (
                    b = 1,
                    c = 1
                ) {
                    to equal_4 {
                        equal(4),
                        not_equal(5)
                    }
                }
            }

            when(a = 3, b = 3, c = 3) as everything_is_three {
                to equal(9)
            }

            when(c = 3) {
                expect(two + c + 10) {
                    let two = 2;

                    to equal(15)
                }
            }

            when all_numbers_are_negative {
                let a = -1;
                let b = -2;
                let c = -3;

                to equal(-6)
            }
        }

        expect(array) {
            when(array = [1, 2, 3]) to equal([1, 2, 3])
        }
    }
}
