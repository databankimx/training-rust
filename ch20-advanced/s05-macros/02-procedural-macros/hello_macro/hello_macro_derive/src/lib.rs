use proc_macro::TokenStream;
use quote::quote;

// The entry point for #[derive(HelloMacro)]. The input is the tokens of the annotated struct or enum.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // syn parses the tokens into a DeriveInput, which knows the type's name. unwrap is acceptable in a sample, but a real macro should report a clearer error.
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // quote! writes Rust code as a template. #name is replaced with the type's name.
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // quote! produces a different token type, so it's converted into the compiler's TokenStream.
    generated.into()
}
