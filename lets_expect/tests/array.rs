#[cfg(test)]
mod lets_expect {
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
        }
    }
}