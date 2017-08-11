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

fn parse_struct_body(body: &Body) -> quote::Tokens {
    match *body {
        Body::Struct(VariantData::Struct(ref fs)) => {
            let fnames = fs.iter().map(|f| &f.ident);
            let fields = fs.iter().map(|f| &f.ident);
           /*  let omg = fs.iter().map(|f| (&f.ident, &f.ty));

            for (_, b) in omg {
                match b {
                    &syn::Ty::Path(_, ref b) => {
                        println!("{}",b.segments.first().unwrap().ident.as_ref());
                    },
                    _ => {

                    }
                }
            } */

            quote! {
                let mut s = String::from("?");

                #(
                    s.push_str(format!("{}={}&", stringify!(#fields), self.#fnames).as_str());
                )*

                return s
            }
        }
        Body::Struct(VariantData::Tuple(ref fs)) =>unimplemented!(),
        Body::Struct(VariantData::Unit) => unimplemented!(),
        Body::Enum(_) => unimplemented!(),
    }
}