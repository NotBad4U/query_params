extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, VariantData};

#[proc_macro_derive(QueryParams)]
pub fn derive_query_params(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;
    let query_params = parse_struct_body(&ast.body);

    let gen = quote! {
        impl QueryParams for #name {
            fn to_query_params(&self) -> String {
                #query_params
            }
        }
    };

    gen.parse().unwrap()
}

fn get_print_fields(fields: &Vec<syn::Field>) -> Vec<quote::Tokens> {
    fields.iter()
        .map(|f| (&f.ident, &f.ty))
        .map(|(ident, ty)| 
            match ty {
                 &syn::Ty::Path(_, ref path) => (ident, extract_type_name(path)),
                 _ => unimplemented!(),
            }
        )
        .map(|(ident, path)| match path {
            "Vec" => quote! {
                s.push_str((format!("{}={}&",
                    stringify!(#ident),
                    self.#ident
                        .iter()
                        .fold(String::new(), |acc, &val| acc + &val.to_string() + ","))
                        .as_str()
                )
                .replace(",&", "&") // remove trailing comma insert by fold
                .as_str())
            },
            _ => quote!{ s.push_str(format!("{}={}&", stringify!(#ident), self.#ident).as_str()) }
        })
        .collect()
}

#[inline]
fn extract_type_name(path: &syn::Path) -> &str {
    path.segments.last().unwrap().ident.as_ref()
}

fn parse_struct_body(body: &Body) -> quote::Tokens {
    match *body {
        Body::Struct(VariantData::Struct(ref fs)) => {
            let vector: Vec<quote::Tokens> = get_print_fields(fs);

            quote! {
                let mut s = String::from("?");

                (#(#vector),*);

                let len_query_params = s.len();
                s.truncate(len_query_params - 1);

                return s;
            }
        }
        Body::Struct(VariantData::Tuple(_)) => panic!("#[derive(QueryParams)] is only defined for structs, not tuple"),
        Body::Struct(VariantData::Unit) => panic!("#[derive(QueryParams)] is only defined for structs, not unit"),
        Body::Enum(_) => panic!("#[derive(QueryParams)] is only defined for structs, not enum"),
    }
}
