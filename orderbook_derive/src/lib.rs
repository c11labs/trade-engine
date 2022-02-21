extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(OrderTrait)]
pub fn order_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Can not parse input TokenStream");

    impl_order(&ast)
}

fn impl_order(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl OrderTrait for #name {
            /* fn print(&self) {
                println!("{:#?}", self);
            } */

            /* fn increase_quantity(&mut self, quantity: u64) -> Result<()> {
                self.current_quantity += quantity;

                Ok(())
            }

            fn decrease_quantity(&mut self, quantity: u64) -> Result<()> {
                if self.current_quantity < quantity {
                    return Err(anyhow!("Not enough quantity in order id {}", self.data.order_id));
                }
                self.current_quantity -= quantity;

                Ok(())
            } */
        }
    };
    gen.into()
}
