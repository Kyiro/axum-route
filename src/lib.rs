use proc_macro::{TokenStream, Span};
use syn::{parse_macro_input, ItemFn};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use quote::quote;

struct Args {
    vars: Vec<syn::Expr>
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let vars = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
        
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Args {
    pub fn get_method(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(0) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No HTTP Method was provided"
            ))
        }
    }
    
    pub fn get_route(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(1) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No Route was provided"
            ))
        }
    }
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let func = parse_macro_input!(input as ItemFn);
    
    let vis = func.vis.clone();
    let ident = func.sig.ident.clone();
    
    let method = args.get_method().unwrap();
    let route = args.get_route().unwrap();
    
    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #vis struct #ident;
        
        impl #ident {
            #vis fn route() -> axum::Router {
                #func
                
                axum::Router::new().route(#route, #method (#ident))
            }
        }
    };
    
    expanded.into()
}