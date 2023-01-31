#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(panic!("I panicked!")) {
            to panic
        }

        expect(true) {
            to not_panic
        }
    }
}
