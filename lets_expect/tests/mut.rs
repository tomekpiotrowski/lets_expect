mod expect {
    use lets_expect::*;

    lets_expect! {
        expect({a += 1; a}) {
            let mut a = 1;

            to equal_2 { 
                equal(2),
                not_equal(1)
            }
        }
    }
}