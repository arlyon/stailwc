#![feature(drain_filter)]
#![feature(box_patterns)]

use std::{cmp::max, collections::HashSet};

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Ident, Item, ItemEnum, ItemMod, Variant};

#[proc_macro_attribute]
pub fn parser(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(input as ItemMod);
    let (_, content) = module.content.as_mut().expect("module must have a root");

    let mut root = content
        .drain_filter(|i| {
            if let Item::Enum(i) = i {
                i.attrs
                    .drain_filter(|a| a.path.segments.last().unwrap().ident.to_string().eq("root"))
                    .count()
                    > 0
            } else {
                false
            }
        })
        .filter_map(|i| if let Item::Enum(e) = i { Some(e) } else { None })
        .next()
        .expect("module must have a root");

    let name = root.ident.clone();

    let mut max_dashes = 0;
    let mut subcommands = vec![];
    let mut has_subsegments = HashSet::new();

    let x = root
        .variants
        .iter_mut()
        .map(|v| {
            let ident = v.ident.clone();

            let kebab = v
                .attrs
                .drain_filter(|v| v.path.get_ident().unwrap() == "rename")
                .next()
                .and_then(|Attribute { tokens, .. }| {
                    let paren = syn::parse2::<syn::ExprParen>(tokens);
                    if let Ok(syn::ExprParen {
                        expr:
                            box syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit),
                                ..
                            }),
                        ..
                    }) = paren
                    {
                        Some(lit.value())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| v.ident.to_string().to_case(Case::Kebab));

            if let Some(f) = v.fields.iter().next() {
                let p = match &f.ty {
                    syn::Type::Path(p) => p,
                    _ => panic!("subcommand must be a struct"),
                };

                let optional = extract_type_from_option(&p.path);

                // find our target type, either from the option or the path
                let target = optional
                    .and_then(|o| match &o {
                        syn::Type::Path(p) => Some(&p.path),
                        _ => None,
                    })
                    .unwrap_or(&p.path)
                    .segments
                    .last()
                    .map(|s| &s.ident);

                // locate the type name, and subcommand enum
                let subcommand = target
                    .and_then(|ident| get_enum_by_ident(content, ident))
                    .expect("enum variant must have a type");

                let format = if v
                    .attrs
                    .drain_filter(|a| a.path.get_ident().unwrap() == "transparent")
                    .count()
                    > 0
                {
                    fmt_transparent
                } else {
                    subcommands.push(ident.clone());
                    if optional.is_none() {
                        has_subsegments.insert(kebab.clone());
                    }
                    fmt_regular
                };

                let subcommands = subcommand.variants.iter().map(|v| {
                    let kebab = format(v, &kebab);
                    let sub_name = subcommand.ident.clone();
                    if let Some((l, _)) = kebab.split_once('-') {
                        has_subsegments.insert(l.to_string());
                    }
                    let sub_ident = v.ident.clone();
                    if optional.is_some() {
                        quote! {
                            #kebab => #name::#ident(Some(#sub_name::#sub_ident)),
                        }
                    } else {
                        quote! {
                            #kebab => #name::#ident(#sub_name::#sub_ident),
                        }
                    }
                });

                if optional.is_some() {
                    let subcommands = subcommands.chain(std::iter::once(quote! {
                        #kebab => #name::#ident(None),
                    }));
                    quote! {
                        #(#subcommands)*
                    }
                } else {
                    quote! {
                        #(#subcommands)*
                    }
                }
            } else {
                max_dashes = max(max_dashes, kebab.chars().filter(|&c| c == '-').count());
                quote! {#kebab => #name::#ident,}
            }
        })
        .collect::<Vec<_>>();

    let has_subsegments = has_subsegments.iter();

    TokenStream::from(quote! {
        mod plugin {
            #root

            #(#content)*

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

                pub fn has_subsegments(name: &str) -> bool {
                    [#(#has_subsegments),*].contains(&name)
                }
            }

            static PLUGINS: phf::Map<&'static str, #name> = phf::phf_map! {
                #(#x)*
            };
        }
    })
}

fn get_enum_by_ident<'a>(content: &'a [Item], ident: &Ident) -> Option<&'a ItemEnum> {
    content
        .iter()
        .filter_map(|i| if let Item::Enum(e) = i { Some(e) } else { None })
        .find(|e| e.ident.eq(ident))
}

fn extract_type_from_option(path: &syn::Path) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        ["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }
    extract_option_segment(path)
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

fn fmt_transparent(v: &Variant, _kebab: &str) -> String {
    v.ident.to_string().to_case(Case::Kebab)
}

fn fmt_regular(v: &Variant, kebab: &str) -> String {
    format!("{kebab}-{}", v.ident.to_string().to_case(Case::Kebab))
}
