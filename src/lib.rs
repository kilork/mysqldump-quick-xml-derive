/*!
Crate `mysqldump-quick-xml-derive` provides a derive macro to convert from mysqldump in xml format to struct using quick-xml.

# Installation

Add following dependency to your `Cargo.toml`:

```toml,ignore
[dependencies]
mysqldump-quick-xml = "0.1"
```

*/
#![recursion_limit = "256"]
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(MysqlDumpQuickXml)]
pub fn mysqldump_quick_xml(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let (fields_declare, fields_set, fields_match_and_set_current, fields_set_none) =
        mysqldump_quick_xml_fields(&input.data);

    let expanded = quote! {

        impl ::mysqldump_quick_xml::MysqlDumpQuickXml for #name {
            fn from_str(xml: &str) -> Vec<Self> {
                use ::mysqldump_quick_xml::quick_xml::Reader;
                use ::mysqldump_quick_xml::quick_xml::events::Event;

                let mut reader = Reader::from_str(xml);
                reader.trim_text(true);

                let mut buf = Vec::new();

                let mut rows = vec![];

                #fields_declare

                let mut current_field = None;

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => match e.name() {
                            b"field" => {
                                let field = e
                                    .attributes()
                                    .map(|x| x.unwrap())
                                    .filter(|x| x.key == b"name")
                                    .next()
                                    .map(|x| x.value)
                                    .unwrap();
                                match field.as_ref() {
                                    #fields_match_and_set_current
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        Ok(Event::End(ref e)) => match e.name() {
                            b"row" => {
                                current_field = None;
                                rows.push(Self {
                                    #fields_set
                                });
                                #fields_set_none
                            }
                            b"field" => {
                                current_field = None;
                            }
                            _ => (),
                        },
                        Ok(Event::Text(e)) => {
                            if let Some(field) = &mut current_field {
                                field.replace(e.unescape_and_decode(&reader).unwrap());
                            }
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("panic at {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }

                    buf.clear();
                }

                rows
            }
        }

    };

    expanded.into()
}

fn mysqldump_quick_xml_fields(data: &Data) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let fields_declare = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        let mut #name: Option<String> = None;
                    }
                });
                let fields_set = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name: #name.clone().unwrap_or_default().into(),
                    }
                });
                let fields_match_and_set_current = fields.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap();
                    let concatenated = format!("{}", name);
                    let varname = syn::LitByteStr::new(concatenated.as_bytes(), name.span());

                    quote! {
                        #varname => current_field = Some(&mut #name),
                    }
                });
                let fields_set_null = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name = None;
                    }
                });

                (
                    quote! {
                        #(#fields_declare)*
                    },
                    quote! {
                        #(#fields_set)*
                    },
                    quote! {
                        #(#fields_match_and_set_current)*
                    },
                    quote! {
                        #(#fields_set_null)*
                    },
                )
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
