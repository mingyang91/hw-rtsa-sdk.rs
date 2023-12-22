extern crate proc_macro;
use quote::quote;

pub struct Definition {
    pub body: proc_macro2::TokenStream,
    pub name: proc_macro2::Ident,
    pub getter: proc_macro2::Ident,
}

pub(crate) fn register_definition(cpp_ident: &proc_macro2::Ident,
                                  rust_ident: &proc_macro2::Ident) -> Definition {
    let register_name = format!("{}_Register", cpp_ident.to_string());
    let register_ident = proc_macro2::Ident::new(&register_name, cpp_ident.span());
    let getter_name = format!("get_{}_register", cpp_ident.to_string());
    let getter_ident = proc_macro2::Ident::new(&getter_name, cpp_ident.span());
    let body = quote! {
        struct #register_ident {
          register: std::sync::RwLock<std::collections::HashMap<*const #cpp_ident, #rust_ident>>,
        }

        impl #register_ident {
            fn get(&self, handler: &*const #cpp_ident) -> Option<#rust_ident> {
                self.register.read().expect("read lock failed").get(handler).map(|h| h.clone())
            }
            fn insert(&self, handler: *const #cpp_ident, h: #rust_ident) {
                self.register.write().expect("write lock failed").insert(handler, h);
            }
            fn remove(&self, handler: &*const #cpp_ident) {
                self.register.write().expect("write lock failed").remove(handler);
            }
        }

        unsafe impl Sync for #register_ident {}
        unsafe impl Send for #register_ident {}

        impl Default for #register_ident {
           fn default() -> Self {
                #register_ident {
                    register: std::sync::RwLock::new(std::collections::HashMap::new()),
                }
           }
        }

        static REGISTER: std::sync::OnceLock<#register_ident> = std::sync::OnceLock::new();

        fn #getter_ident() -> &'static #register_ident {
            REGISTER.get_or_init(Default::default)
        }
    };

    Definition {
        body,
        name: register_ident,
        getter: getter_ident
    }
}