//! Bundles of code generated for each `#[extensions(...)]` name.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Fields, Ident, ItemStruct, Token, punctuated::Punctuated};

// ---------------------------------------------------------------------------
// Supported extensions
// ---------------------------------------------------------------------------

const SUPPORTED: &[&str] = &[
    "children",
    "key",
    "style",
    "event_handlers",
    "accessibility",
];

fn dependencies_of(name: &str) -> &'static [&'static str] {
    match name {
        _ => &[],
    }
}

// ---------------------------------------------------------------------------
// Attribute argument parsing
// ---------------------------------------------------------------------------

pub struct ExtensionNames {
    pub names: Vec<String>,
}

impl Parse for ExtensionNames {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            names: parsed.into_iter().map(|i| i.to_string()).collect(),
        })
    }
}

pub fn parse_names(attr: TokenStream2) -> syn::Result<Vec<String>> {
    Ok(syn::parse2::<ExtensionNames>(attr)?.names)
}

// ---------------------------------------------------------------------------
// Field-name lookup
// ---------------------------------------------------------------------------

pub fn field_names_for(name: &str) -> &'static [&'static str] {
    match name {
        "children" => &["children"],
        "key" => &["key"],
        "style" => &["style"],
        "event_handlers" => &["event_handlers"],
        "accessibility" => &["accessibility"],
        _ => &[],
    }
}

pub fn collect_field_names(names: &[String]) -> Vec<String> {
    names
        .iter()
        .flat_map(|n| field_names_for(n).iter().map(|s| s.to_string()))
        .collect()
}

// ---------------------------------------------------------------------------
// Extension bundle
// ---------------------------------------------------------------------------

struct Extension {
    fields: Vec<TokenStream2>,
    inits: Vec<TokenStream2>,
    locals: Vec<TokenStream2>,
    impls: Vec<TokenStream2>,
}

fn extension_for(
    name: &str,
    struct_name: &Ident,
    variant_field_names: &[Ident],
) -> syn::Result<Extension> {
    let ext = match name {
        "children" => Extension {
            fields: vec![quote! {
                /// Child elements attached to this component.
                pub children: ::std::vec::Vec<::freyacn::prelude::Element>
            }],
            inits: vec![quote! { children: ::std::vec::Vec::new() }],
            locals: vec![quote! {
                let children: &::std::vec::Vec<::freyacn::prelude::Element> = &self.children;
            }],
            impls: vec![quote! {
                impl ::freyacn::prelude::ChildrenExt for #struct_name {
                    fn get_children(&mut self) -> &mut ::std::vec::Vec<::freyacn::prelude::Element> {
                        &mut self.children
                    }
                }
            }],
        },

        "key" => Extension {
            fields: vec![quote! {
                /// The element's diff key.
                pub key: ::freyacn::prelude::DiffKey
            }],
            inits: vec![quote! { key: ::freyacn::prelude::DiffKey::None }],
            locals: vec![quote! {
                let key: &::freyacn::prelude::DiffKey = &self.key;
            }],
            impls: vec![quote! {
                impl ::freyacn::prelude::KeyExt for #struct_name {
                    fn write_key(&mut self) -> &mut ::freyacn::prelude::DiffKey {
                        &mut self.key
                    }
                }
                impl #struct_name {
                    fn render_key(&self) -> DiffKey {
                        self.key.clone().or(self.default_key())
                    }
                }
            }],
        },

        // -------------------------------------------------------------------
        // style  →  freyacn::StyleExt
        // -------------------------------------------------------------------
        "style" => {
            // The `style` field itself is a plain `Style` — the user's
            // override layer. `#[component]` merges variant-derived styles
            // into a local `style` binding at render time.
            //
            // If there are no variants, we emit a simple reference binding.
            // Otherwise we compute a merged value into a local and bind
            // that.
            let locals = if variant_field_names.is_empty() {
                vec![quote! {
                    let style: &::freyacn::prelude::Style = &self.style;
                }]
            } else {
                let merges = variant_field_names.iter().map(|field| {
                    quote! {
                        __freyacn_base = __freyacn_base.merge(
                            ::freyacn::prelude::Variant::resolve(&self.#field)
                        );
                    }
                });
                vec![quote! {
                    let __freyacn_merged_style: ::freyacn::prelude::Style = {
                        let mut __freyacn_base =
                            <::freyacn::prelude::Style as ::core::default::Default>::default();
                        #(#merges)*
                        __freyacn_base.merge(self.style.clone())
                    };
                    let style: &::freyacn::prelude::Style = &__freyacn_merged_style;
                }]
            };

            Extension {
                fields: vec![quote! {
                    /// Accumulated user overrides.
                    ///
                    /// Holds the style applied via [`StyleExt`] methods.
                    /// Variant-derived styles are merged on top of this at
                    /// render time — see the component macro's generated
                    /// `render` body for details.
                    pub style: ::freyacn::prelude::Style
                }],
                inits: vec![quote! {
                    style: <::freyacn::prelude::Style as ::core::default::Default>::default()
                }],
                locals,
                impls: vec![quote! {
                    impl ::freyacn::prelude::StyleExt for #struct_name {
                        fn get_style(&mut self) -> &mut ::freyacn::prelude::Style {
                            &mut self.style
                        }
                    }
                }],
            }
        }

        "event_handlers" => Extension {
            fields: vec![quote! {
                /// Event-handler registry.
                pub event_handlers:
                    ::freyacn::prelude::FxHashMap<::freyacn::prelude::EventName, ::freyacn::prelude::EventHandlerType>
            }],
            inits: vec![quote! {
                event_handlers:
                    <::freyacn::prelude::FxHashMap<_, _> as ::core::default::Default>::default()
            }],
            locals: vec![quote! {
                let event_handlers: &::freyacn::prelude::FxHashMap<
                    ::freyacn::prelude::EventName,
                    ::freyacn::prelude::EventHandlerType,
                > = &self.event_handlers;
            }],
            impls: vec![quote! {
                impl ::freyacn::prelude::EventHandlersExt for #struct_name {
                    fn get_event_handlers(
                        &mut self,
                    ) -> &mut ::freyacn::prelude::FxHashMap<
                        ::freyacn::prelude::EventName,
                        ::freyacn::prelude::EventHandlerType,
                    > {
                        &mut self.event_handlers
                    }
                }
            }],
        },

        // -------------------------------------------------------------------
        // accessibility  →  freyacn::AccessibilityExt
        // -------------------------------------------------------------------
        "accessibility" => Extension {
            fields: vec![quote! {
                /// Accessibility data for this element.
                ///
                /// Populated via the [`AccessibilityExt`] builder methods
                /// (`a11y_role`, `a11y_alt`, `a11y_focusable`, …) and
                /// forwarded to the primitive when the component renders.
                pub accessibility: ::freyacn::prelude::AccessibilityData
            }],
            inits: vec![quote! {
                accessibility: <::freyacn::prelude::AccessibilityData as ::core::default::Default>::default()
            }],
            locals: vec![quote! {
                let accessibility: &::freyacn::prelude::AccessibilityData = &self.accessibility;
            }],
            impls: vec![quote! {
                impl ::freyacn::prelude::AccessibilityExt for #struct_name {
                    fn get_accessibility_data(
                        &mut self,
                    ) -> &mut ::freyacn::prelude::AccessibilityData {
                        &mut self.accessibility
                    }
                }
            }],
        },

        other => {
            let valid = SUPPORTED
                .iter()
                .map(|n| format!("`{n}`"))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(Error::new(
                Span::call_site(),
                format!("Unsupported extension `{other}`. Valid names: {valid}."),
            ));
        }
    };
    Ok(ext)
}

// ---------------------------------------------------------------------------
// Collector
// ---------------------------------------------------------------------------

pub struct ExtensionsCode {
    pub field_defs: Vec<TokenStream2>,
    pub field_inits: Vec<TokenStream2>,
    pub local_bindings: Vec<TokenStream2>,
    pub trait_impls: Vec<TokenStream2>,
}

pub fn collect(
    struct_name: &Ident,
    names: &[String],
    variant_field_names: &[Ident],
) -> syn::Result<ExtensionsCode> {
    let mut out = ExtensionsCode {
        field_defs: Vec::new(),
        field_inits: Vec::new(),
        local_bindings: Vec::new(),
        trait_impls: Vec::new(),
    };

    for name in names {
        for dep in dependencies_of(name) {
            if !names.iter().any(|n| n == dep) {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "`{name}` requires `{dep}` to also be listed in \
                         `ext(...)`. Add it, e.g. `ext({dep}, {name})`.",
                    ),
                ));
            }
        }

        let ext = extension_for(name, struct_name, variant_field_names)?;
        out.field_defs.extend(ext.fields);
        out.field_inits.extend(ext.inits);
        out.local_bindings.extend(ext.locals);
        out.trait_impls.extend(ext.impls);
    }

    Ok(out)
}

// ---------------------------------------------------------------------------
// Standalone `#[extensions(...)]`
// ---------------------------------------------------------------------------

pub fn parse_extensions(attr: TokenStream2, item: TokenStream2) -> syn::Result<TokenStream2> {
    let item_struct: ItemStruct = syn::parse2(item)?;
    let struct_name = item_struct.ident.clone();

    let names = parse_names(attr)?;
    let code = collect(&struct_name, &names, &[])?;

    let Fields::Named(named) = &item_struct.fields else {
        return Err(Error::new_spanned(
            &struct_name,
            "`#[extensions]` can only be used on structs with named fields",
        ));
    };

    let existing_fields = named.named.iter();
    let extra_fields = &code.field_defs;
    let impls = &code.trait_impls;

    let attrs = &item_struct.attrs;
    let vis = &item_struct.vis;
    let generics = &item_struct.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ext_list = names
        .iter()
        .map(|n| format!("* `{n}`"))
        .collect::<Vec<_>>()
        .join("\n");
    let generated_note =
        format!("## Extensions\n\nThis struct has been extended with:\n\n{ext_list}");

    Ok(quote! {
        #(#attrs)*
        #[doc = ""]
        #[doc = #generated_note]
        #vis struct #struct_name #generics #where_clause {
            #(#existing_fields,)*
            #(#extra_fields,)*
        }

        #(#impls)*

        #[allow(unused)]
        const _: fn() = || {
            fn _assert_generics #impl_generics () #where_clause {}
            let _ = _assert_generics::<#ty_generics>;
        };
    })
}
