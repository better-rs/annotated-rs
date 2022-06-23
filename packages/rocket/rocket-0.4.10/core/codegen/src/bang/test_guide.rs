use std::path::Path;
use std::error::Error;

use proc_macro::TokenStream;
use devise::{syn::{self, Ident, LitStr}, Result};

use crate::syn_ext::syn_to_diag;
use crate::proc_macro2::TokenStream as TokenStream2;

pub fn _macro(input: TokenStream) -> Result<TokenStream> {
    let root = syn::parse::<LitStr>(input.into()).map_err(syn_to_diag)?;
    let modules = entry_to_modules(&root)
        .map_err(|e| root.span().unstable().error(format!("failed to read: {}", e)))?;

    Ok(quote_spanned!(root.span() =>
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        mod test_site_guide { #(#modules)* }
    ).into())
}

fn entry_to_modules(pat: &LitStr) -> std::result::Result<Vec<TokenStream2>, Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("MANIFEST_DIR");
    let full_pat = Path::new(&manifest_dir).join(&pat.value()).display().to_string();

    let mut modules = vec![];
    for path in glob::glob(&full_pat).map_err(|e| Box::new(e))? {
        let path = path.map_err(|e| Box::new(e))?;
        let name = path.file_name()
            .and_then(|f| f.to_str())
            .map(|name| name.trim_matches(|c| char::is_numeric(c) || c == '-')
                .replace('-', "_")
                .replace('.', "_"))
            .ok_or_else(|| "invalid file name".to_string())?;

        let ident = Ident::new(&name, pat.span());
        let full_path = Path::new(&manifest_dir).join(&path).display().to_string();
        modules.push(quote_spanned!(pat.span() =>
            #[doc(include = #full_path)]
            struct #ident;
        ))
    }

    Ok(modules)
}
