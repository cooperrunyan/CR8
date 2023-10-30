use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, parse_str, Ident, LitStr, Path, Token, Visibility};

extern crate phf;

///  ## Examples
///
///     modules! {
///         $VIS static "modules/" $NAME = {
///             std::{
///                 gfx::{
///                     box,
///                     grid
///                 }
///             }
///         };
///     }
struct Modules {
    visibility: Visibility,
    name: Ident,
    basepath: LitStr,
    tree: Tree,
}

#[derive(Clone)]
struct Tree {
    items: Punctuated<Node, Token![,]>,
}

#[derive(Clone)]
enum Node {
    Tree(Ident, Tree),
    Node(Ident),
}

impl Parse for Modules {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![static]>()?;
        let basepath: LitStr = input.parse()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        let tree: Tree = input.parse()?;
        Ok(Self {
            visibility,
            name,
            basepath,
            tree,
        })
    }
}

impl Parse for Tree {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(Self {
            items: content.parse_terminated(Node::parse, Token![,])?,
        })
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let item = input.call(Ident::parse_any)?;
        if input.parse::<Token![::]>().is_ok() {
            let tree: Tree = input.parse()?;
            Ok(Node::Tree(item, tree))
        } else {
            Ok(Node::Node(item))
        }
    }
}

trait Flatten {
    fn flatten(&self, parent: &str) -> Vec<String>;
}

impl Flatten for Node {
    fn flatten(&self, parent: &str) -> Vec<String> {
        match self {
            Node::Node(item) => vec![format!("{}::{}", parent, item)],
            Node::Tree(item, tree) => {
                let current = format!("{}::{}", parent, item);
                let mut nodes = vec![format!("{current}::mod")];
                nodes.append(&mut tree.flatten(&current));
                nodes
            }
        }
    }
}

impl Flatten for Tree {
    fn flatten(&self, parent: &str) -> Vec<String> {
        self.items
            .iter()
            .flat_map(|it| it.flatten(parent))
            .collect::<Vec<_>>()
    }
}

fn gen(node: Node, base: &str, travelled: &[String]) -> proc_macro2::TokenStream {
    match node {
        Node::Node(id) => {
            let path = format!("{}/{}", travelled.join("/"), id)
                .trim_start_matches('/')
                .to_string();

            let realpath = format!("{base}{path}.asm");
            let key = path.replace('/', "::");
            let fi = quote!(include_str!(#realpath));
            let doc = format!("`{key}`\n\n`{realpath}`");

            quote! {
                #[doc = #doc]
                pub mod #id {
                    pub static __FILE__: &'static str = #fi;
                }
            }
        }
        Node::Tree(id, items) => {
            let mut travelled = travelled.to_vec();

            let path = format!("{}/{}", travelled.join("/"), id)
                .trim_start_matches('/')
                .to_string();

            let realpath = format!("{base}{path}/mod.asm");
            let key = path.replace('/', "::");

            let doc = format!("`{key}`\n\n`{realpath}`");

            travelled.push(id.to_string());

            let children = items
                .items
                .into_iter()
                .map(|item| gen(item, base, &travelled));

            let fi = quote! { include_str!(#realpath) };
            quote! {
                #[doc = #doc]
                pub mod #id {
                    pub static __FILE__: &'static str = #fi;

                    #( #children )*
                }
            }
        }
    }
}

pub fn modules(input: TokenStream) -> TokenStream {
    let modules = parse_macro_input!(input as Modules);
    let modname = format_ident!("_{}", modules.name);
    let id = modules.name;
    let vis = modules.visibility;
    let base = modules.basepath.value();

    let entries = modules
        .tree
        .items
        .into_iter()
        .map(|item| (item.flatten("_"), gen(item, &base, &[])));

    let (keys, children): (Vec<Vec<String>>, Vec<proc_macro2::TokenStream>) = entries.unzip();
    let keys = keys.into_iter().flat_map(|k| k.into_iter()).map(|k| {
        let stripped = k.strip_suffix("::mod").unwrap_or(&k);
        let stripped = stripped.strip_prefix("_::").unwrap_or(stripped);

        let modpath = format!("{modname}::{stripped}::__FILE__");
        let key = LitStr::new(stripped, Span::mixed_site());

        let m = parse_str::<Path>(&modpath).unwrap();
        quote! {
            #key => #m,
        }
    });

    quote! {
        #[allow(non_snake_case)]
        mod #modname {
            #( #children )*
        }

        #vis static #id: phf::Map<&'static str, &'static str> = phf::phf_map! {
            #( #keys )*
        };
    }
    .into()
}
