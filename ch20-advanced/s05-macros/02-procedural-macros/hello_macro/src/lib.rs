// The trait that the derive macro implements. It's defined here, not in the macro crate, because users need it in scope.
pub trait HelloMacro {
    fn hello_macro();
}
