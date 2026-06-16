use heck::{ToSnakeCase, ToTitleCase};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Type};

#[derive(Copy, Clone)]
enum PropertyType {
    F32,
    I32,
    Bool,
    String,
}

impl PropertyType {
    fn new(ty: &Type) -> Self {
        match ty {
            Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    match ident.to_string().as_str() {
                        "f32" => PropertyType::F32,
                        "i32" => PropertyType::I32,
                        "bool" => PropertyType::Bool,
                        "String" => PropertyType::String,
                        _ => panic!("Unsupported property type"),
                    }
                } else {
                    panic!("Unsupported property type");
                }
            }
            _ => {
                panic!("Unsupported property type");
            }
        }
    }
}

fn get_default_value(attribute: &Attribute) -> TokenStream {
    if let Some(TokenTree::Group(group)) = attribute.tokens.clone().into_iter().next() {
        if group.delimiter() != Delimiter::Parenthesis {
            return quote! { ::std::default::Default::default() };
        }

        let tokens = group.stream().into_iter().collect::<Vec<_>>();

        if tokens.len() != 3 {
            return quote! { ::std::default::Default::default() };
        }

        if let (TokenTree::Ident(ident), TokenTree::Punct(punct), TokenTree::Literal(lit)) =
            (&tokens[0], &tokens[1], &tokens[2])
        {
            if ident == "default" && punct.as_char() == '=' {
                return TokenTree::from(lit.clone()).into();
            } else {
                panic!("Only (default = ..) supported")
            }
        }

        // This case is for bools with default value true/false
        if let (TokenTree::Ident(ident), TokenTree::Punct(punct), TokenTree::Ident(default)) =
            (&tokens[0], &tokens[1], &tokens[2])
        {
            if ident == "default" && punct.as_char() == '=' {
                return TokenTree::from(default.clone()).into();
            } else {
                panic!("Only (default = ..) supported")
            }
        }
    }

    quote! { ::std::default::Default::default() }
}

struct Property<'a> {
    ident: &'a Ident,
    property_type: PropertyType,
    default_value: TokenStream,
}

#[proc_macro_derive(Properties, attributes(property))]
pub fn derive_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.to_string();
    let snake_case_name = name.to_snake_case();

    let mut properties = Vec::new();

    let mut all_fields_are_properties = true;

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                if let Some(attribute) = field.attrs.iter().find(|a| {
                    if let Some(ident) = a.path.get_ident() {
                        if ident == "property" {
                            return true;
                        }
                    }
                    false
                }) {
                    if let Some(ident) = &field.ident {
                        properties.push(Property {
                            ident,
                            property_type: PropertyType::new(&field.ty),
                            default_value: get_default_value(attribute),
                        });
                    } else {
                        all_fields_are_properties = false;
                    }
                } else {
                    all_fields_are_properties = false;
                }
            }
        } else {
            panic!("Only supports Structs with named fields");
        }
    } else {
        panic!("Only supports Structs");
    }

    let struct_ident = Ident::new(&name, input.ident.span());
    let internal_mod_ident =
        Ident::new(&format!("__{snake_case_name}_internal"), input.ident.span());

    let mut load_properties = quote! {};
    let mut save_properties = quote! {};
    let mut gui_properties = quote! {};
    let mut defaulted_fields = quote! {};

    for property in properties.iter() {
        let property_ident = property.ident;
        let property_name = property.ident.to_string();
        let property_ui_name = property_name.to_title_case();
        let default_value = &property.default_value;

        match property.property_type {
            PropertyType::F32 => {
                load_properties.extend(quote! {
                    if let Some(value) = api.read_float(#property_name) {
                        self.#property_ident = value;
                    } else {
                        api.write_float(#property_name, #default_value);
                        self.#property_ident = #default_value;
                    }
                });
                save_properties.extend(quote! {
                    api.write_float(#property_name, self.#property_ident);
                });
                gui_properties.extend(quote! {
                    if api.drag_float(
                        #property_ui_name,
                        0.01,
                        0.0,
                        0.0,
                        "%.02f",
                        1.0,
                        &mut self.#property_ident,
                    ) {
                        api.write_float(#property_name, self.#property_ident);
                    }
                });
                defaulted_fields.extend(quote! {
                    #property_ident: #default_value,
                });
            }
            PropertyType::I32 => {
                load_properties.extend(quote! {
                    if let Some(value) = api.read_int(#property_name) {
                        self.#property_ident = value;
                    } else {
                        api.write_int(#property_name, #default_value);
                        self.#property_ident = #default_value;
                    }
                });
                save_properties.extend(quote! {
                    api.write_int(#property_name, self.#property_ident);
                });
                gui_properties.extend(quote! {
                    if api.drag_int(
                        #property_ui_name,
                        0.01,
                        0,
                        0,
                        "%.0f",
                        &mut self.#property_ident,
                    ) {
                        api.write_int(#property_name, self.#property_ident);
                    }
                });

                defaulted_fields.extend(quote! {
                    #property_ident: #default_value,
                });
            }
            PropertyType::Bool => {
                load_properties.extend(quote! {
                    if let Some(value) = api.read_int(#property_name) {
                        self.#property_ident = if value > 0 { true } else { false };
                    } else {
                        let i = if #default_value { 1 } else { 0 };
                        api.write_int(#property_name, i);
                        self.#property_ident = #default_value;
                    }
                });
                save_properties.extend(quote! {
                    {
                        let i = if self.#property_ident { 1 } else { 0 };
                        api.write_int(#property_name, i);
                    }
                });
                gui_properties.extend(quote! {
                    if api.checkbox(
                        #property_ui_name,
                        &mut self.#property_ident,
                    ) {
                        let i = if self.#property_ident { 1 } else { 0 };
                        api.write_int(#property_name, i);
                    }
                });
                defaulted_fields.extend(quote! {
                    #property_ident: #default_value,
                });
            }
            PropertyType::String => {
                load_properties.extend(quote! {
                    if let Some(value) = api.read_string(#property_name) {
                        self.#property_ident = value;
                    } else {
                        api.write_string(#property_name, #default_value);
                        self.#property_ident = #default_value.to_string();
                    }
                });
                save_properties.extend(quote! {
                    api.write_string(#property_name, &self.#property_ident);
                });
                gui_properties.extend(quote! {
                    if api.input_text(#property_ui_name, &mut self.#property_ident) {
                        api.write_string(#property_name, &self.#property_ident);
                    }
                });
                defaulted_fields.extend(quote! {
                    #property_ident: #default_value.to_string(),
                });
            }
        }
    }

    let default_impl = if all_fields_are_properties {
        quote! {
            impl Default for crate::#struct_ident {
                fn default() -> Self {
                    Self {
                        #defaulted_fields
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {

        mod #internal_mod_ident {

            #default_impl

            impl ::oden_plugin_rs::OdenPluginWithProperties for crate::#struct_ident {
                fn load_properties(&mut self, api: &::oden_plugin_rs::InitParams) {
                    #load_properties
                }

                fn save_properties(&mut self, api: &::oden_plugin_rs::UpdateParams) {
                    #save_properties
                }

                fn gui_properties(&mut self, api: &::oden_plugin_rs::GuiParams) {
                    #gui_properties
                }
            }

            impl crate::#struct_ident {
                pub fn as_oden_plugin_with_properties(
                    &mut self,
                ) -> Option<&mut dyn ::oden_plugin_rs::OdenPluginWithProperties> {
                    Some(self as _)
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
