pub struct ScopeCall<F: FnOnce()> {
    pub c: Option<F>,
}

impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

#[allow(unused_macros)]
macro_rules! expr {
    ($e: expr) => {
        $e
    };
}

#[allow(unused_macros)]
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = ScopeCall {
            c: Some(|| -> () { expr!({ $($data)* })})
        };
    )
}

#[allow(unused_imports)]
pub(crate) use defer;
#[allow(unused_imports)]
pub(crate) use expr;
