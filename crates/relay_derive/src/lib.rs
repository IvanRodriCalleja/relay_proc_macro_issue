use quote::quote;
use proc_macro::{self, TokenStream};

use syn::{Data, DeriveInput, parse_macro_input};



#[proc_macro_attribute]
pub fn relay_node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    if let Data::Struct(_) = ast.data {
        let name = &ast.ident;

        let gen = quote! {            
            impl relay_spec::RelayNode for #name {
                fn get(id: String) -> juniper::FieldResult<Self> {
                    // TODO: Implement this method to refetch the object from the database.
                    unimplemented!()
                }
            }
        };

        gen.into()
    } else {
        let error = quote! {
            compile_error!("relay_node can only be applied to structs");
        };
        error.into()
    }
}
