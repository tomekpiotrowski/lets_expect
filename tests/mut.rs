#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect({a += 1; a}) {
            let mut a = 1;

            to equal_2 {
                equal(2),
                not_equal(1)
            }
        }

        expect(a += 1) {
            when(mut a: i64 = 1) {
                to change(a.clone()) { from(1), to(2) }
            }
        }
    }
}
