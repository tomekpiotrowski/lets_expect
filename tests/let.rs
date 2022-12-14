#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(a) {
            let a: u8 = 1;

            to equal(1)
        }

        expect(a + b) {
            let a = 3;
            let b = 1;

            to equal(4)
        }

        expect(a + b + c == 5) {
            let a = 3;
            let b = 1;
            let c = 1;

            to be_true
        }

        expect(multiplied_by_2) {
            let value = 5;
            let multiplied_by_2 = value * 2;

            when(value = 10) {
                to equal(20)
            }

            when(value = value * 3) {
                to equal(30)
            }

            when(value: u128 = 1267650600228229401496703205376) {
                to equal(2535301200456458802993406410752)
            }
        }

    }
}
