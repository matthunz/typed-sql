extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

#[proc_macro_derive(Table)]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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
                #name: typed_sql::field::Field::<#ident, #ty>,
            }
        });

        let default_fields = fields.named.iter().map(|field| {
            let name = &field.ident;
            quote! {
                #name: typed_sql::field::Field::new(stringify!(#name)),
            }
        });

        let table_name = {
            let mut s = ident.to_string().to_lowercase();
            s.push('s');
            Ident::new(&s, Span::call_site())
        };

        let expanded = quote! {
            struct #fields_ident {
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
                #name: typed_sql::join::Joined<#g, typed_sql::join::Inner, #ty>
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
                #generic: typed_sql::select::Predicate
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

            impl<#impl_generics> typed_sql::join::JoinSelect for #join_ident<#join_generics> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
