use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta,
    MetaNameValue, PathArguments, Type,
};

#[proc_macro_derive(DialoguerParser, attributes(arg, prompt, clap, command))]
pub fn dialoguer_parser_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    // create a shadow struct that is the same as the normal one, but with each field wrapped in an Option
    let shadow_struct_name = format_ident!("{}Optionals", struct_name);
    // first copy the attrs from clap to the shadow struct
    let clap_struct_attrs: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|attr| is_clap_attr(attr))
        .collect();

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(named_fields) = data.fields {
            named_fields.named
        } else {
            panic!("DialoguerParser only supports structs with named fields");
        }
    } else {
        panic!("DialoguerParser only supports structs");
    };

    // Booleans (flags) need to be supported differently, in clap's default behavior, if a flag is not specified, it defaults to false, but because we prompt any field that is not specified, flag do not work like intended, so we need to exclude them from being prompted

    // We split the fields into to groups:
    // - Fields without Option (ex: age: u32) => those will be prompted if missing
    // - Fields with Option (ex: output_file: Option<String>), or flag (booleans) => those will be optional fields and not be prompted

    let shadow_struct_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        let clap_attrs: Vec<&Attribute> =
            f.attrs.iter().filter(|attr| is_clap_attr(attr)).collect();

        if is_option_type(ty) || is_bool_type(ty) {
            // This fields is optional
            quote! {
                #(#clap_attrs)*
                #name: #ty
            }
        } else {
            // This field is required
            quote! {
                #(#clap_attrs)*
                #name: Option<#ty>
            }
        }
    });

    let field_prompters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if is_option_type(ty) || is_bool_type(ty) {
            // If the fields is optional, we don't prompt it
            quote! {
                let #name = opts.#name;
            }
        } else {
            // If the fields is required, we prompt it
            let prompt_text =
                get_prompt(&f.attrs).unwrap_or_else(|| format!("Enter {}", name.as_ref().unwrap()));

            quote! {
                let #name = opts.#name.unwrap_or_else(||{
                    ::dialoguer::Input::<#ty>::new()
                        .with_prompt(#prompt_text)
                        .interact_text()
                        .unwrap()
                });
            }
        }
    });

    let field_inits = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name:#name
        }
    });

    let output = quote! {
        // shadow struct
        #[derive(::clap::Parser)]
        #(#clap_struct_attrs)*
        struct #shadow_struct_name {
            #(#shadow_struct_fields),*
        }

        impl #struct_name {
            pub fn parse() -> Self {
                let opts = #shadow_struct_name::parse();

                #(#field_prompters)*

                Self {
                    #(#field_inits),*
                }
            }
        }

    };
    return output.into();
}

fn is_clap_attr(attr: &Attribute) -> bool {
    if let Some(ident) = attr.path().get_ident() {
        let s = ident.to_string();
        matches!(s.as_str(), "arg" | "clap" | "command" | "doc")
    } else {
        false
    }
}

/// Checks if a type is an Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(path_type) = ty {
        if let Some(last_seg) = path_type.path.segments.last() {
            // is type an Option and does it have arguments
            return last_seg.ident.to_string() == "Option"
                && matches!(last_seg.arguments, PathArguments::AngleBracketed(_));
        }
    }
    return false;
}

fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
            return type_path.path.segments[0].ident == "bool";
        }
    }
    return false;
}

fn get_prompt(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("prompt") {
            if let Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(string),
                        ..
                    }),
                ..
            }) = &attr.meta
            {
                return Some(string.value());
            }
        }
    }
    None
}
