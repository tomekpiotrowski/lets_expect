#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

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
    }
}
