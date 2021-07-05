extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Ident, Lit, Meta,
    MetaNameValue,
};

#[proc_macro_derive(Table, attributes(typed))]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let table_name = parse_table_attributes(&input.attrs);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = &input.ident;
        let fields_ident = {
            let mut s = ident.to_string();
            s.push_str("Fields");
            Ident::new(&s, Span::call_site())
        };

        let struct_fields = fields.named.iter().map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote! {
                #name: typed_sql::types::Field::<#ident, #ty>,
            }
        });

        let default_fields = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! {
                #name: typed_sql::types::Field::new(stringify!(#name)),
            }
        });

        let table_name = {
            let s = if table_name.is_some() {
                table_name.unwrap()
            } else {
                ident.to_string().to_lowercase()
            };
            Ident::new(&s, Span::call_site())
        };

        let expanded = quote! {
            pub struct #fields_ident {
              #(#struct_fields)*
            }

            impl Default for #fields_ident {
                fn default() -> Self {
                    Self {
                        #(#default_fields)*
                    }
                }
            }

            impl typed_sql::Table for #ident {
                const NAME: &'static str = stringify!(#table_name);

                type Fields = #fields_ident;
            }
        };

        TokenStream::from(expanded)
    } else {
        todo!()
    }
}

#[proc_macro_derive(Join)]
pub fn join(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = input.ident;
        let fields_ident = format_ident!("{}Fields", ident);

        let struct_fields = fields.named.iter().map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            quote! {
                #name: <#ty as typed_sql::Table>::Fields
            }
        });

        let mut fields = fields.named.iter();
        let table = &fields.next().unwrap().ty;

        let join_ident = format_ident!("{}Join", ident);
        let join_fields = fields.clone().map(|field| {
            let name = &field.ident;
            let g = field.ident.as_ref().unwrap().to_string().to_uppercase();
            let g = format_ident!("{}", g);
            let ty = &field.ty;
            quote! {
                #name: typed_sql::query::select::join::Joined<#g, typed_sql::query::select::join::Inner, #ty>
            }
        });

        let generics = fields.map(|field| {
            Ident::new(
                &field.ident.as_ref().unwrap().to_string().to_uppercase(),
                Span::call_site(),
            )
        });

        let join_generics = generics.clone().map(|generic| {
            quote! {
                #generic
            }
        });
        let join_generics = quote! {
            #(#join_generics),*
        };

        let impl_generics = generics.clone().map(|generic| {
            quote! {
                #generic: typed_sql::query::Predicate
            }
        });
        let impl_generics = quote! {
            #(#impl_generics),*
        };

        let expanded = quote! {
            #[derive(Default)]
            struct #fields_ident {
                #(#struct_fields),*
            }

            struct #join_ident<#join_generics> {
                #(#join_fields),*
            }

            impl<#impl_generics> typed_sql::Join<(#join_generics)> for #ident {
                type Table = #table;
                type Fields = #fields_ident;
                type Join = #join_ident<#join_generics>;
            }

            impl<#impl_generics> typed_sql::query::select::join::JoinSelect for #join_ident<#join_generics> {
                type Table = #table;
                type Fields = #fields_ident;

                fn write_join_select(&self, sql: &mut String) {
                    self.post.write_join(sql);
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        todo!()
    }
}

#[proc_macro_derive(Insertable)]
pub fn insertable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = &input.ident;

        let write_columns = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! { sql.push_str(stringify!(#name)); }
        });

        let write_values = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! { self.#name.write_primative(sql); }
        });

        let expanded = quote! {
            impl typed_sql::Insertable for #ident {
                fn write_columns(sql: &mut String) {
                    #(#write_columns){sql.push(',');}*
                }

                fn write_values(&self, sql: &mut String) {
                    use typed_sql::types::Primitive;
                    #(#write_values){sql.push(',');}*
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        todo!()
    }
}

#[proc_macro_derive(Binding)]
pub fn binding(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = &input.ident;
        let bindings = format_ident!("{}Bindings", ident);

        let bind_fields = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! { #name: typed_sql::types::Bind }
        });

        let binds = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! { #name: binder.bind() }
        });

        let values = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! { self.#name.write_primative(sql); }
        });

        let expanded = quote! {
            struct #bindings {
                #(#bind_fields),*
            }

            impl typed_sql::Binding for #ident {
                type Bindings = #bindings;

                fn bindings(binder: &mut typed_sql::types::bind::Binder) -> Self::Bindings {
                    #bindings {
                        #(#binds),*
                    }
                }

                fn write_types(_sql: &mut String) {}

                fn write_values(&self, sql: &mut String) {
                    use typed_sql::types::Primitive;
                    #(#values){sql.push(',');}*;
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        todo!()
    }
}

#[proc_macro_derive(Queryable)]
pub fn queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = &input.ident;
        let columns = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! {
                sql.push_str(stringify!(#name));
            }
        });

        let expanded = quote! {
            impl typed_sql::Queryable for #ident {
                fn write_queryable(sql: &mut String) {
                    #(#columns)(sql.push(','))*
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        todo!()
    }
}

fn parse_table_attributes(input: &[Attribute]) -> Option<String> {
    let mut table_name = None;
    for attr in input {
        match attr.parse_meta().ok() {
            None => return None,
            Some(meta) => match meta {
                Meta::List(list) if list.path.is_ident("typed") => {
                    for value in list.nested.iter() {
                        match value {
                            syn::NestedMeta::Meta(meta) => match meta {
                                Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(val),
                                    ..
                                }) if path.is_ident("table_name") => {
                                    table_name = Some(val.value().to_uppercase());
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
    }

    table_name
}
