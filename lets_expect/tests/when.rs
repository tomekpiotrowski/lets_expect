#[cfg(test)]
mod expect {
    use lets_expect::*;

    lets_expect! {
        expect(a + b + c) {
            when(let a = 2;) {
                when (
                    let b = 1;
                    let c = 1;
                ) {
                    to equal_4 {
                        equal(4),
                        not_equal(5)
                    }
                }
            }

            when(let c = 3;) {
                expect(two + c + 10) {
                    let two = 2;

                    to equal(15)
                }
            }
        }
    }
}