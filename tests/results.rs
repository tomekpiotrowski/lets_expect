#[cfg(test)]
mod tests {
    use lets_expect::lets_expect;

    lets_expect! {
        expect(Ok(1) as Result<i32, String>) {
            to be_ok {
                equal(Ok(1)),
                be_ok
            }
        }

        expect(Err(2) as Result<i32, i32>) {
            to be_err {
                equal(Err(2)),
                be_err
            }
        }
    }
}
