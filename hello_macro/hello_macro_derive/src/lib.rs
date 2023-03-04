use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
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

// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
//     let fields = match &ast.data {
//         Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
//         _ => panic!("this derive macro only works on structs with named fields"),
//     };
//
//     let name = &ast.ident;
//     let field_name = &fields.into_iter().next().unwrap().ident;
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}!", #field_name);
//             }
//         }
//     };
//
//
//     gen.into()
// }
