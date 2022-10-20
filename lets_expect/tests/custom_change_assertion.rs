use lets_expect::{AssertionError, AssertionResult};

mod point;

fn by_multiplying_by(x: i32) -> impl Fn(i32, i32) -> AssertionResult {
    move |before, after| {
        if after == before * x {
            Ok(())
        } else {
            Err(AssertionError::new(vec![format!(
                "Expected {} to be multiplied by {} to be {}, but it was {} instead",
                before,
                x,
                before * x,
                after
            )]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lets_expect::lets_expect;

    lets_expect! {
        expect(a *= 5) {
            let mut a = 5;

            to change(a) by_multiplying_by(5)
        }
    }
}
