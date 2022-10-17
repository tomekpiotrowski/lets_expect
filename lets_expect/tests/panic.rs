struct IPanic {
    pub should_panic: bool
}

impl IPanic {
    pub fn new() -> Self {
        Self { should_panic: false }
    }

    pub fn panic(&self) {
        panic!("panic");
    }

    pub fn no_panic(&self) {
    }

    pub fn panic_if_should(&self) {
        if self.should_panic {
            panic!();
        }
    }
}

mod expect {
    use lets_expect::*;
    use crate::IPanic;

    lets_expect! {
        expect(panic!("I panicked!")) {
            to panic
        }

        expect(2) {
            to not_panic
        }

        expect(IPanic::new()) {
            to panic_and_not_panic {
                have(panic()) panic,
                have(no_panic()) not_panic
            }
        }

        expect("unrelated") {
            let i_panic = IPanic::new();

            to panic_or_not_panic { 
                make(i_panic.panic()) panic,
                make(i_panic.no_panic()) not_panic
            }
        }

        expect(i_panic.should_panic = true) {
            let mut i_panic = IPanic::new();

            to change(i_panic.panic_if_should()) { from_not_panic, to_panic }
        }

        expect(i_panic.should_panic = false) {
            let mut i_panic = {
                let mut i_panic = IPanic::new();
                i_panic.should_panic = true;
                i_panic
            };

            to change(i_panic.panic_if_should()) { from_panic, to_not_panic }
        }
    }
}