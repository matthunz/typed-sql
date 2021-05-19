extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

/// ```
/// use typed_sql::Table;
///
/// #[derive(Table)]
/// struct User {
///     id: i64
/// }
///
/// assert_eq!(User::NAME, "users");
/// ```
#[proc_macro_derive(Table)]
pub fn my_macro(input: TokenStream) -> TokenStream {
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
                #name: typed_sql::field::Field<#ident, #ty>,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
