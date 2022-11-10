#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(empty_vec) {
            let empty_vec: Vec<String> = vec![];
        }

        expect(vec![1, 2, 3]) {
            to contain_expected_values {
                have(len()) equal(3)
            }

            to have(mut iter()) all(be_greater_than(0))
            to have(iter().next()) equal(Some(&1))
        }

        expect(mut vec![1, 2, 3]) {
            to have(remove(1)) equal(2)
        }

        expect(mut vec.iter()) {
            let vec = vec![1, 2, 3];
            to all(be_greater_than(0))
        }

        expect(vec.remove(1)) {
            when(mut vec = vec![1, 2, 3]) {
                to equal(2)
            }
        }
    }
}
