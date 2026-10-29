## Procedural Macros

Procedural macros take Rust code as input, run a Rust function on it, and produce Rust code as output. Unlike declarative macros, they can analyze the code they're given, such as its names and
fields. There are three kinds:

* **Custom `derive` macros**, which add code for a `#[derive(...)]` attribute on a struct or enum.
* **Attribute-like macros**, which define new attributes that can go on other items, such as functions.
* **Function-like macros**, which look like function calls, such as `sql!(SELECT * FROM posts)`.

Each one is a function that takes a `TokenStream` and returns a `TokenStream`. A `TokenStream` is a sequence of tokens. A procedural macro must be in its own crate, with the `proc-macro` crate type.
This step uses a derive, so the project has three crates. The macro crate lives in a subfolder of this step, and the main project depends on it by path.

**The layout:**

```text
02-procedural-macros
├── Cargo.toml          (pancakes, the program that uses the derive)
├── src/main.rs
└── hello_macro
    ├── Cargo.toml      (hello_macro, the trait)
    ├── src/lib.rs
    └── hello_macro_derive
        ├── Cargo.toml  (proc-macro crate, uses syn and quote)
        └── src/lib.rs
```

**The trait** is an ordinary trait, in `hello_macro`:

```rust,ignore
pub trait HelloMacro {
    fn hello_macro();
}
```

**The derive** reads the type's name, and writes an implementation of the trait for it. The `syn` crate parses the input into a `DeriveInput`, and the `quote!` macro writes the output as a template, with `#name`
replaced by the name:

```rust,ignore
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
```

**The program** uses the derive, and writes no implementation itself:

```rust,ignore
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

The two `HelloMacro` names don't clash. The trait is in the type namespace, and the derive is in the macro namespace. The output is:

```
Hello, Macro! My name is Pancakes!
```

**Attribute-like and function-like macros** follow the same pattern. An attribute-like macro takes two `TokenStream` values, the contents of the attribute and the item it's attached to. A function-like macro takes the
tokens between its parentheses, as in the `sql!` example. They're written in their own crates, in the same way as this derive.

---

### Try it in the runner

Press **Run**. The first build downloads `syn` and `quote`, which takes a while. The output should be `Hello, Macro! My name is Pancakes!`.

The comments in `src/main.rs` suggest deleting the line that imports the trait. The derive's generated code names `HelloMacro`, and the compiler reports that it can't find the trait in scope.

---

*For more detail, see section 20.5 "Macros" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-05-macros.html).*
