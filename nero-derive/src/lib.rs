#![crate_type = "proc-macro"]
#![recursion_limit = "192"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;

use syn::{punctuated::Punctuated, Attribute, MetaNameValue, Token};

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(proc_macro2::Span::call_site() => $($t)*))
}

#[proc_macro_derive(Model, attributes(model))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Couldn't parse item");
    let result = match ast.data {
        syn::Data::Struct(ref s) => model_for_struct(&ast, &s.fields, None),
        _ => panic!("work only with struct"),
    };

    result.into()
}

fn model_for_struct(
    ast: &syn::DeriveInput,
    fields: &syn::Fields,
    variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => new_impl(ast, &fields.named, variant),
        _ => panic!("Work only with named struct"),
    }
}

fn new_impl(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
    _variant: Option<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let scheme = Scheme::parse(name.to_string(), fields).to_token_stream();

    my_quote! {
        #[nero::async_trait]
        impl ::nero::db::model::Manager for #name {
            fn table_name() -> String {
               ::nero::db::model::format_table_name(stringify!(#name))
            }

            async fn get(id: ::nero::db::model::Id) -> ::nero::error::Result<Self> {
                ::nero::db::model::SurrealDriver::get(Self::thing_from_id(id)).await
            }

            fn scheme() -> ::nero::db::scheme::Scheme {
                // println!("Scheme: {}", stringify!(#scheme));

                // println!("attr: {}", #test_attr);
                // #(
                //     println!("attr={attr}", attr = stringify!(#attrs));
                // )*
                // #(
                //     println!(
                //         "key={key}, type={type_name}",
                //         key = #keys,
                //         type_name = stringify!(#types)
                //     );
                // )*

                #scheme
            }

            async fn create(&mut self) {
                let id =
                   ::nero::db::model::SurrealDriver::create(self.id.clone(), Self::table_name(), self).await;
                self.id = Some(Self::thing_from_id(id));
            }

            async fn delete(self) -> Self {
                ::nero::db::model::SurrealDriver::delete(self.id.clone().expect("Thing must be set")).await
            }

            async fn delete_with_id(id: ::nero::db::model::Id) -> Self {
                ::nero::db::model::SurrealDriver::delete(Self::thing_from_id(id)).await
            }

            async fn update(&self) {
               ::nero::db::model::SurrealDriver::update(self.id.clone().expect("Thing must be set"), self)
                    .await;
            }
        }

    }
}

struct Scheme {
    name: String,
    fields: Vec<Field>,
}

impl Scheme {
    fn parse(name: String, fields: &Punctuated<syn::Field, Token![,]>) -> Self {
        Self {
            name,
            fields: fields.iter().map(Field::parse).collect(),
        }
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let fields: Vec<proc_macro2::TokenStream> = self
            .fields
            .iter()
            .map(|field| field.to_token_stream())
            .collect();
        let fields = my_quote! { #(#fields, )* };

        my_quote! {
            ::nero::db::scheme::Scheme {
                name: (#name).to_string(),
                fields: vec![#fields]
            }
        }
    }
}

struct Field {
    name: String,
    tp: String,
    is_option: bool,
    attrs: Attributes,
}

impl Field {
    fn parse(field: &syn::Field) -> Self {
        let name: String = field.ident.clone().unwrap().to_string();
        let mut tp = field.ty.to_token_stream().to_string().replace(' ', "");
        let mut is_option = false;
        let mut attrs = Attributes::default();

        if tp.starts_with("Option<") {
            tp = tp.replace("Option<", "").replace('>', "");
            is_option = true
        }

        let model_attr: Vec<&Attribute> = field
            .attrs
            .iter()
            .filter(|attr| attr.meta.path().to_token_stream().to_string() == "model")
            .collect();
        if let Some(attr) = model_attr.first() {
            attrs = Attributes::parse(attr)
        }

        Self {
            name,
            tp,
            is_option,
            attrs,
        }
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let tp = &self.tp;
        let is_option = &self.is_option;
        let attrs = self.attrs.to_token_stream();

        my_quote! {
            ::nero::db::scheme::Field {
                name: (#name).to_string(),
                tp: (#tp).to_string(),
                is_option: #is_option,
                attrs: #attrs
            }
        }
    }
}

#[derive(Default)]
struct Attributes {
    max_length: Option<proc_macro2::TokenStream>,
    default: Option<proc_macro2::TokenStream>,
}

impl Attributes {
    fn parse(attr: &Attribute) -> Self {
        let attr_parse: Punctuated<MetaNameValue, Token![,]> =
            attr.parse_args_with(Punctuated::parse_terminated).unwrap();
        let mut _self = Self::default();

        for attr in attr_parse {
            let name = attr.path.get_ident().unwrap().to_string();

            match name.as_str() {
                "max_length" => _self.max_length = Some(attr.value.to_token_stream()),
                "default" => _self.default = Some(attr.value.to_token_stream()),
                _ => panic!("Attribute {name} not support"),
            }
        }

        _self
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let max_length = match &self.max_length {
            Some(val) => my_quote! { Some(#val) },
            None => my_quote! { None },
        };

        let default = match &self.default {
            Some(val) => my_quote! { Some(#val) },
            None => my_quote! { None },
        };

        my_quote! {
            ::nero::db::scheme::Attributes {
                max_length: #max_length,
                default: #default
            }
        }
    }
}
