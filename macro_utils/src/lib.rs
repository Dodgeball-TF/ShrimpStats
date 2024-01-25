// This macro is used to generate the `EventHandler` enum.
// It takes a list of PascalCase event names and generates an enum with the same names.
// the inner struct is also generated with the same name, but in snake_case.
// the inner struct uses the format crate::models::events::snake_case_name::Inner

// So for example, if you pass in `ServerRegister`, the macro will generate:
// use crate::models::events;
//
// pub enum EventHandler {
//   ServerRegister(events::server_register::Inner),
// }
//

// The inner structs already exist so they just have to be referenced.
// The enum is generated from the list of names passed in.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn UnclassifiedEvent(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => panic!("EventType can only be derived for enums"),
    };

    let mut event_names = Vec::new();

    for variant in variants {
        let name = variant.ident;
        event_names.push(name);
    }

    let mut fields = Vec::new();

    for name in event_names.iter() {
        let snake_case_name = snake_case(&name.to_string());
        let event_folder_name = snake_case_name.split('_').next().unwrap().to_lowercase();
        let inner_name = format!("events::{}::{}::Inner", event_folder_name, snake_case_name);
        let inner_name = syn::parse_str::<syn::Type>(&inner_name).unwrap();

        let event_handler = quote! {
            #name(#inner_name),
        };

        fields.push(event_handler);
    }

    // We should also generate an impl for the enum.
    // Each inner implements the Event trait. It has a single method, handle().
    // EventMaster also implements the Event trait. It has a single method, handle().
    // The handle() method for EventMaster will match on the enum and call the handle() method for the inner.

    let event_handler = quote! {
        use crate::models::events;

        pub enum UnclassifiedEvent {
            #(#fields)*
        }

        impl UnclassifiedEvent {
            pub fn handle(&self) {
                match self {
                    #(
                        UnclassifiedEvent::#event_names(inner) => inner.handle(),
                    )*
                }
            }
        }
    };

    event_handler.into()
}

fn snake_case(s: &str) -> String {
    // Turn a pacal case string into snake case
    // Example: ServerRegister -> server_register
    let mut snake_case = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(c.to_lowercase().next().unwrap());
        } else {
            snake_case.push(c);
        }
    }

    snake_case
}
