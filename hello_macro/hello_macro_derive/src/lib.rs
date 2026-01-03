use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    
    eprintln!("INPUT:\n{}\n", input);

    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    let out = impl_hello_macro(&ast);

    eprintln!("OUTPUT:\n{out}\n");
    out
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {    
    eprintln!("AST:\n{:#?}\n", ast);

    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    
    eprintln!("generated:\n{generated}\n");

    generated.into()
}
