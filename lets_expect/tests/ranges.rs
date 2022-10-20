#[cfg(test)]
mod tests {
    use lets_expect::*;

    lets_expect! {
        expect(1..4) {
            to contain_expected_values {
                have(contains(&2)) equal(true),
                have(contains(&5)) not_equal(true),
                have(len()) equal(3)
                //contain(&2),
                //not_contain(&7),
                //not_be_empty,
            }
        }
    }
}
