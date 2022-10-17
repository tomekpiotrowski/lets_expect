#[cfg(test)]
mod expect {
    use lets_expect::*;

    lets_expect! {
        expect(empty_vec) {
            let empty_vec: Vec<String> = vec![];
        }

        expect(vec![1, 2, 3]) {
            to contain_expected_values {
                have(len()) equal(3)
            }
        }
    }
}