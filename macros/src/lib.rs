extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitInt, LitStr, Token};

struct HashedStrInput {
    input_string: LitStr,
    output_size: LitInt,
}

impl Parse for HashedStrInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_string: LitStr = input.parse()?;
        let _: Token![,] = input.parse()?;
        let output_size: LitInt = input.parse()?;
        Ok(HashedStrInput {
            input_string,
            output_size,
        })
    }
}

#[proc_macro]
pub fn hashed_str(input: TokenStream) -> TokenStream {
    let HashedStrInput {
        input_string,
        output_size,
        ..
    } = parse_macro_input!(input as HashedStrInput);

    let input_string_value = input_string.value();
    let output_size_value = output_size.base10_parse::<usize>().unwrap();

    let mut hasher = Shake256::default();
    hasher.update(input_string_value.as_bytes());

    let mut output_bytes = vec![0u8; output_size_value];
    hasher.finalize_xof().read(&mut output_bytes);

    let output = quote! {
        &[#(#output_bytes),*] as &'static [u8; #output_size_value]
    };

    TokenStream::from(output)
}
