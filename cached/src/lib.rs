use proc_macro::TokenStream;
use syn::{
    parse_macro_input, 
    Signature, 
    ItemFn, 
    FnArg, 
    ReturnType
};
use quote::{quote, format_ident, ToTokens};

type TokenStream2 = proc_macro2::TokenStream;

/* Unwrap optional tokens to either an empty token or the actual token*/
fn unwrap_opt_token<T: ToTokens>(t: Option<T>) -> TokenStream2 {
    match t {
        Some(t) => quote! {#t},
        None => quote! {}
    }
}

#[proc_macro_attribute]
pub fn cached(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { 
        sig: Signature { 
            unsafety,
            ident, 
            generics: _, 
            inputs, 
            output,
            ..
        },
        block,
        ..
    } = parse_macro_input!(input as ItemFn);

    // Unwrap optional tokens to either an empty token or the actual token
    //let constness = unwrap_opt_token(constness);
    //let asyncness = unwrap_opt_token(asyncness);
    let unsafety = unwrap_opt_token(unsafety);
    

    // Split `inputs` into idents and types
    let (arg_ident, arg_type): (Vec<_>, Vec<_>) = inputs.iter().filter_map(|arg| match arg {
        FnArg::Receiver(_) => None,
        FnArg::Typed(t) => Some((t.pat.clone(), t.ty.clone()))
    }).unzip();

    // Define the output as either the unit type or the explicit return type
    let return_type = match output {
        ReturnType::Default => syn::parse_str("()").unwrap(),
        ReturnType::Type(_, t) => t
    };

    // Define the name of the lazy_static hashmap
    let cache_ident = format_ident!("{}_FUNCTION_CACHE", ident.to_string().to_uppercase());

    // Define type signature for hashmap declaration as well as arguments for accessing the map
    let key_type = quote! {(#(#arg_type),*)};
    let key_ident = quote! {(#(#arg_ident),*)};

    // Declare a global lazy_static hashmap
    let cache = quote! {
        lazy_static::lazy_static! {
            static ref #cache_ident: std::sync::Mutex<std::collections::HashMap<#key_type, #return_type>> = {
                std::sync::Mutex::new(std::collections::HashMap::new())
            };
        }
    };

    // Modify the actual function
    let cached_function = quote! {
        #unsafety fn #ident(#inputs) -> #return_type {
            // If a result from a previous call is stored, return the stored result
            if let Some(&res) = #cache_ident.lock().unwrap().get(&#key_ident) {
                return res;
            }
            // Store the result of the function block in `res` and insert it in the cache map 
            let res = #block;
            #cache_ident.lock().unwrap().insert(#key_ident, res);
            res
        }
    };

    quote! {
        #cache
        #cached_function
    }.into()
}
