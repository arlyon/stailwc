#![feature(drain_filter)]

use std::cmp::max;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_attribute]
pub fn parser(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut variants = parse_macro_input!(input as ItemEnum);
    let name = variants.ident.clone();

    let mut max_dashes = 0;
    let mut subcommands = vec![];

    let x = variants
        .variants
        .iter_mut()
        .map(|v| {
            let ident = v.ident.clone();
            let kebab = v.ident.to_string().to_case(Case::Kebab);

            if let Some(f) = v.fields.iter().next() {
                if let syn::Type::Path(p) = &f.ty {
                    let transparent = v
                        .attrs
                        .drain_filter(|a| a.path.get_ident().unwrap() == "transparent")
                        .count()
                        > 0;

                    let type_name = p.path.segments.last().map(|s| s.ident.to_string());

                    // todo: determine method to get these subcommands

                    return if transparent {
                        // transparent commands should be placed directly in the hash table
                        quote! {}
                    } else {
                        // other commands should be handled seperately
                        subcommands.push(ident.clone());
                        if type_name.as_deref() != Some("Option") {
                            println!("UNSUPPORTED SUBCOMMAND {}", ident);
                            quote! {}
                        } else {
                            quote! {#kebab => #name::#ident(None),}
                        }
                    };
                }
            }

            max_dashes = max(max_dashes, kebab.chars().filter(|&c| c == '-').count());
            quote! {#kebab => #name::#ident,}
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #variants

        impl std::str::FromStr for #name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                PLUGINS.get(s).copied().ok_or(())
            }
        }

        impl #name {
            pub const fn max_dashes() -> usize {
                #max_dashes
            }

            pub fn has_subcommand(&self) -> bool {
                match self {
                    #(#name::#subcommands(_))|* => true,
                    _ => false
                }
            }
        }

        static PLUGINS: phf::Map<&'static str, #name> = phf::phf_map! {
            #(#x)*
        };
    })
}
