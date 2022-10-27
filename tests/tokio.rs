#[cfg(all(test, feature = "tokio"))]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! { #tokio_test
        let value = 5;
        let spawned = tokio::spawn(async move {
            value
        });

        expect(spawned.await) {
            to match_pattern!(Ok(5))
        }
    }
}
