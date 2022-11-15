use proc_macro::TokenStream;
use quote::quote;
use syn;

/// using the `derive(HelloMacro)` attribute will call the `hello_macro_derive` function
/// the parsing function uses the `syn::parse` function to parse the code into a syntax tree. the
/// function then calls the implementing function
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // Construct a representation of Rust code as a syntax tree `DeriveInput` from a string `TokenStream`
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
