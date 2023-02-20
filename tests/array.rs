#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    struct Array<T> {
        data: Vec<T>,
    }

    lets_expect! {
        expect([] as [u8; 0]) {
            to have(len()) equal(0)
        }

        expect([1, 2 ,3]) {
            to contain_valid_elements {
                have(len()) equal(3),
                have(contains(&1)) be_true
            }
            to have(mut iter()) all(be_greater_than(0))
            to have(mut iter()) any(be_less_or_equal_to(1))
        }

        expect(Array { data: vec![1, 2, 3] }) {
            to have(data[0]) equal(1)
            to have(data[1..].to_vec()) equal(vec![2, 3])
        }

        expect(Array { data: vec![vec![1,2,3], vec![0]]}) as array_of_non_copy_type {
            let unrelated_array = Array { data: vec![vec!["a"]] };
            to have(&data[1]) equal(vec![0])
            to make(&unrelated_array.data[0]) equal(vec!["a"])
        }
    }
}
