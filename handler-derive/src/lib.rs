extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, PathArguments, TypeBareFn};

#[proc_macro_derive(Handler)]
pub fn handler_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = match input.data {
        Data::Struct(data) => {
            let expand_callback_functions = data.fields.iter()
                .filter_map(|f| {
                    match parse_ffi_callback(&f.ty) {
                        Some(func) => Some(expand_ffi_callback(&f.ident, func)),
                        None => None,
                    }
                });
            let expand_external_functions = data.fields.iter()
                .filter_map(|f| {
                    match parse_ffi_callback(&f.ty) {
                        Some(func) => Some(expand_external_function(&f.ident, func)),
                        None => None,
                    }
                });
            let expand_callback_register = data.fields.iter()
                .filter_map(|f| {
                    match parse_ffi_callback(&f.ty) {
                        Some(_) => Some(expand_callback_register(&f.ident)),
                        None => None,
                    }
                });
            quote! {
                #(#expand_external_functions)*
                struct HandlerRegister {
                  register: std::sync::RwLock<std::collections::HashMap<*const #name, HRTSAHandler>>,
                }

                impl HandlerRegister {
                    fn get(&self, handler: &*const #name) -> Option<HRTSAHandler> {
                        self.register.read().expect("read lock failed").get(handler).map(|h| h.clone())
                    }
                    fn insert(&self, handler: *const #name, h: HRTSAHandler) {
                        self.register.write().expect("write lock failed").insert(handler, h);
                    }
                    fn remove(&self, handler: &*const #name) {
                        self.register.write().expect("write lock failed").remove(handler);
                    }
                }

                unsafe impl Sync for HandlerRegister {}
                unsafe impl Send for HandlerRegister {}

                impl Default for HandlerRegister {
                   fn default() -> Self {
                        HandlerRegister {
                            register: std::sync::RwLock::new(std::collections::HashMap::new()),
                        }
                   }
                }

                static REGISTER: std::sync::OnceLock<HandlerRegister> = std::sync::OnceLock::new();

                fn get_register() -> &'static HandlerRegister {
                    REGISTER.get_or_init(Default::default)
                }

                pub trait Handler {
                    fn raw_ptr(&self) -> *mut #name;
                    fn log(&self, msg: &str) {}
                    #(#expand_callback_functions)*
                }

                #[derive(Clone)]
                pub struct HRTSAHandler {
                    inner: std::sync::Arc<std::sync::Mutex<Box<dyn Handler>>>,
                }

                impl HRTSAHandler {
                    pub fn new<T: Handler + 'static>(handler: T) -> Self {
                        let mut raw_ptr = handler.raw_ptr();
                        unsafe {
                            #(#expand_callback_register)*
                        }
                        let key = raw_ptr as *const #name;
                        let this = HRTSAHandler {
                            inner: std::sync::Arc::new(std::sync::Mutex::new(Box::new(handler))),
                        };
                        get_register().insert(key, this.clone());
                        this
                    }

                    pub fn raw_ptr(&self) -> *mut #name {
                        self.inner.lock().expect("lock failed").raw_ptr() as *mut #name
                    }
                }

                impl Drop for HRTSAHandler {
                    fn drop(&mut self) {
                        if 1 == std::sync::Arc::strong_count(&self.inner) {
                            let key = self.inner.lock().expect("lock failed").raw_ptr();
                            get_register().remove(&(key as *const #name));
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

fn expand_ffi_callback(ident: &Option<proc_macro2::Ident>, func: &TypeBareFn) -> proc_macro2::TokenStream {
    let names = func.inputs
        .iter()
        .skip(1)
        .map(|arg| arg.name.as_ref().map(|i| &i.0));
    let types = func.inputs
        .iter()
        .skip(1)
        .map(|arg| &arg.ty);
    let ret = &func.output;
    let expanded = quote! {
        fn #ident(&mut self #(, #names: #types)*) #ret {
            self.log(format!("{}: {}", stringify!(#ident), "not implemented").as_str());
            Default::default()
        }
    };

    // dbg!(expanded.to_string());
    expanded
}

fn extern_function_suffix(ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(&format!("{}_entry", ident), ident.span())
}

fn expand_external_function(ident: &Option<proc_macro2::Ident>, func: &TypeBareFn) -> proc_macro2::TokenStream {
    let names = func.inputs.iter()
        .map(|arg| arg.name.as_ref().map(|i| &i.0));
    let types = func.inputs.iter()
        .map(|arg| &arg.ty);
    let names2 = names.clone().skip(1);
    let ret = &func.output;
    let ident = ident.as_ref().expect("ident not found");
    let suffixed_ident = extern_function_suffix(ident);

    let expanded = quote! {
        extern "C" fn #suffixed_ident(#(#names: #types, )*) #ret {
            let register = get_register();
            if let Some(handler) = register.get(&self_) {
                let mut guard = handler.inner.lock().expect("lock failed");
                guard.#ident(#(#names2, )*)
            } else {
                eprintln!("{:p}: handler not found", self_);
                Default::default()
            }
        }
    };

    // dbg!(expanded.to_string());
    expanded
}

fn expand_callback_register(ident: &Option<proc_macro2::Ident>) -> proc_macro2::TokenStream {
    let ident = ident.as_ref().expect("ident not found");
    let suffixed_ident = extern_function_suffix(ident);
    let expanded = quote! { (*raw_ptr).#ident = Some(#suffixed_ident); };

    expanded
}