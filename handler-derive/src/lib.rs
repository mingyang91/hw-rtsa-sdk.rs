mod register;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, PathArguments, TypeBareFn};
use crate::register::{Definition, register_definition};

#[proc_macro_derive(FFICallbacks)]
pub fn handler_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = match input.data {
        Data::Struct(data) => {

            let callback_agent_name = format!("{}_FFIAgent", &name.to_string());
            let callback_agent_ident = proc_macro2::Ident::new(&callback_agent_name, name.span());

            let callback_protocol_name = format!("{}_FFIProtocol", &name.to_string());
            let callback_protocol_ident = proc_macro2::Ident::new(&callback_protocol_name, name.span());
            let Definition {
                body: expand_register_definition,
                name: register_ident,
                getter: getter_ident
            } = register_definition(&name, &callback_agent_ident);

            let parse_ffi_callbacks = data.fields.iter()
                .map(|f| parse_ffi_callback(&f.ty));
            let fields_zip_callbacks = data.fields.iter().zip(parse_ffi_callbacks);
            let expand_callback_functions = fields_zip_callbacks
                .clone()
                .filter_map(|(f, ffi)| {
                    match ffi {
                        Some(func) => Some(expand_ffi_callback(&f.ident.as_ref().expect("field no name"), func)),
                        None => None,
                    }
                });
            let expand_external_functions = fields_zip_callbacks
                .clone()
                .filter_map(|(f, ffi)| {
                    match ffi {
                        Some(func) => Some(expand_external_function(&f.ident.as_ref().expect("field no name"), &getter_ident, func)),
                        None => None,
                    }
                });
            let expand_callback_register = fields_zip_callbacks
                .filter_map(|(f, ffi)| {
                    match ffi {
                        Some(_) => Some(expand_callback_register(&f.ident.as_ref().expect("field no name"))),
                        None => None,
                    }
                });

            quote! {
                #(#expand_external_functions)*
                #expand_register_definition
                pub trait #callback_protocol_ident {
                    fn log(&self, msg: &str) {}
                    #(#expand_callback_functions)*
                }

                #[derive(Clone)]
                pub struct #callback_agent_ident {
                    inner: std::sync::Arc<std::sync::Mutex<Box<dyn #callback_protocol_ident>>>,
                    pub raw_ptr: *mut #name,
                }

                impl #callback_agent_ident {
                    pub fn new<T: #callback_protocol_ident + 'static>(handler: T) -> Self {
                        let mut raw_ptr = unsafe { createHandler() };
                        unsafe {
                            #(#expand_callback_register)*
                        }
                        let key = raw_ptr as *const #name;
                        let this = #callback_agent_ident {
                            inner: std::sync::Arc::new(std::sync::Mutex::new(Box::new(handler))),
                            raw_ptr
                        };
                        #getter_ident().insert(key, this.clone());
                        this
                    }
                }

                impl Drop for #callback_agent_ident {
                    fn drop(&mut self) {
                        if 1 == std::sync::Arc::strong_count(&self.inner) {
                            #getter_ident().remove(&(self.raw_ptr as *const #name));
                        }
                    }
                }
            }
        }
        _ => panic!("only support struct"),
    };
    TokenStream::from(expanded)
}

fn parse_ffi_callback(ty: &syn::Type) -> Option<&TypeBareFn> {
    let syn::Type::Path(pth) = ty else {
        return None;
    };

    let segments = &pth.path.segments;
    if 3 != segments.len() {
        return None;
    };

    let fst = &segments[0];
    let snd = &segments[1];
    let trd = &segments[2];

    if fst.ident.to_string() != "std"
        || snd.ident.to_string() != "option"
        || trd.ident.to_string() != "Option"
    {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &trd.arguments else {
        return None;
    };

    if 1 != args.args.len() {
        return None;
    }

    let fst = &args.args[0];

    let syn::GenericArgument::Type(ty) = fst else {
        return None;
    };

    let syn::Type::BareFn(bare_fn) = ty else {
        return None;
    };

    Some(bare_fn)
}

fn expand_ffi_callback(ident: &proc_macro2::Ident, func: &TypeBareFn) -> proc_macro2::TokenStream {
    let names = func.inputs
        .iter()
        .skip(1)
        .map(|arg| arg.name.as_ref().map(|i| &i.0));
    let types = func.inputs
        .iter()
        .skip(1)
        .map(|arg| &arg.ty);
    let ret = &func.output;
    let rust_name = to_rust_name(&ident);
    let expanded = quote! {
        fn #rust_name(&mut self #(, #names: #types)*) #ret {
            self.log(format!("{}: {}", stringify!(#rust_name), "not implemented").as_str());
            Default::default()
        }
    };

    // dbg!(expanded.to_string());
    expanded
}

fn extern_function_suffix(ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(&format!("{}_entry", ident), ident.span())
}

fn expand_external_function(ident: &proc_macro2::Ident,
                            getter: &proc_macro2::Ident,
                            func: &TypeBareFn) -> proc_macro2::TokenStream {
    let names = func.inputs.iter()
        .map(|arg| arg.name.as_ref().map(|i| &i.0));
    let types = func.inputs.iter()
        .map(|arg| &arg.ty);
    let names2 = names.clone().skip(1);
    let ret = &func.output;
    let rust_name = to_rust_name(ident);
    let suffixed_ident = extern_function_suffix(ident);

    let expanded = quote! {
        extern "C" fn #suffixed_ident(#(#names: #types, )*) #ret {
            let register = #getter();
            if let Some(handler) = register.get(&self_) {
                let mut guard = handler.inner.lock().expect("lock failed");
                guard.#rust_name(#(#names2, )*)
            } else {
                eprintln!("{:p}: #callback_protocol_ident not found", self_);
                Default::default()
            }
        }
    };

    // dbg!(expanded.to_string());
    expanded
}

fn expand_callback_register(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let suffixed_ident = extern_function_suffix(ident);
    let expanded = quote! { (*raw_ptr).#ident = Some(#suffixed_ident); };

    expanded
}

// input: _functionNameFromCpp
// output: function_name_from_cpp
fn to_rust_name(ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    let mut name = String::new();
    for c in ident.to_string().chars() {
        if c.is_uppercase() {
            name.push('_');
            name.push(c.to_lowercase().next().expect("impossible"));
        } else {
            name.push(c);
        }
    }
    if name.starts_with('_') {
        name.remove(0);
    }
    proc_macro2::Ident::new(&name, ident.span())
}