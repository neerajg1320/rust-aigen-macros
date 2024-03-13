extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, ItemFn};

// Derive Procedural Macro
// Macro name is between the braces: HelloWorld
#[proc_macro_derive(HelloWorld)]
pub fn helloworld_object_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Name would contain the name of the struct or the enum on which the macro would be applied
    let name = input.ident;

    // Generate the code to paste into the user's program
    let expanded = quote! {
        imple HelloWorld for #name {
            fn hello_world() {
                println!("Hello World");
            }
        }
    };

    // Hand the output tokens back to the compile
    TokenStream::from(expanded)
}

// Attribute like procedural macro
// The function name is the macro name
#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // Create string representation of function
    let function_str: String = format!("{}", input_fn.to_token_stream());

    // Define a new function with the same signature as the input funciton
    let fn_ident: proc_macro2::Ident = input_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs;
    let fn_generics: syn::Generics = input_fn.sig.generics;

    // Generate output function
    let output: proc_macro2::TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}

// Functional like procedural macro
// The function name is the macro name
#[proc_macro]
pub fn create_struct(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as Ident);

    // Construct a string representation of the struct definition
    let name = &input;
    let expanded = quote! {
        struct #name {
            x: i32,
            y: i32,
        }
    };

    // Return the generated struct as TokenStream
    expanded.into()
}
