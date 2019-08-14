#![forbid(unsafe_code)]
#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{export::*, *};

// TODO remove .unwrap calls
// TODO make limiting consistent

const DEFAULT_LIMIT: usize = 100;

#[proc_macro_derive(ArangoBuilder)]
pub fn arango_builder(item: TokenStream) -> TokenStream {
    _arango_builder(parse_macro_input![item as ItemStruct]).into()
}

fn _arango_builder(struct_definition: ItemStruct) -> TokenStream2 {
    let struct_name = &struct_definition.ident;
    let builder_name_precursor = || format!("{}ArangoBuilder", struct_name);
    let builder_name = Ident::new(&builder_name_precursor(), Span::call_site());
    let builder_factory_name = Ident::new(
        &format!("{}ArangoBuilderFactory", struct_name),
        Span::call_site(),
    );
    let default_limit = quote![serde_json::to_value(&#DEFAULT_LIMIT).unwrap()]; // safe to unwrap

    // Struct specific methods for field based filtering
    let condition_qs = struct_definition
        .fields
        .iter()
        .flat_map(|field| {
            [
                ("eq", "=="),
                ("ne", "!="),
                ("gt", ">"),
                ("lt", "<"),
                ("ge", ">="),
                ("le", "<="),
                ("in", "IN"),
                ("not_in", "NOT IN"),
            ]
            .iter()
            .map(move |(op_name, op)| {
                let id = field_id(&field);
                let ty = &field.ty;
                let call = format!("item.{}", id);
                let fn_name = Ident::new(&format!("{}_{}", id, op_name), Span::call_site());
                let bn = Ident::new(&builder_name_precursor(), Span::call_site());

                match op_name {
                    &"in" | &"not_in" =>
                        quote![
                            fn #fn_name(self, values: &[#ty]) -> #bn<Conditional> {
                                let mut new_bind_vars = self.bind_vars;
                                let bind_var_name = format!("filterVar{}", new_bind_vars.len());

                                new_bind_vars
                                    .insert(
                                        bind_var_name.clone(),
                                        serde_json::to_value(&values).unwrap()
                                    );

                                let mut new_raw_query = self.raw_query;
                                new_raw_query.push(format!("{} {} @{}", #call, #op, bind_var_name));

                                #bn {
                                    query_type: self.query_type,
                                    tag: Conditional,
                                    bind_vars: new_bind_vars,
                                    raw_query: new_raw_query,
                                }
                            }
                        ],
                    _ =>
                        quote![
                            fn #fn_name(self, value: &#ty) -> #bn<Conditional> {
                                let mut new_bind_vars = self.bind_vars;
                                let bind_var_name = format!("filterVar{}", new_bind_vars.len());

                                new_bind_vars
                                    .insert(bind_var_name.clone(), serde_json::to_value(&value).unwrap());

                                let mut new_raw_query = self.raw_query;
                                new_raw_query.push(format!("{} {} @{}", #call, #op, bind_var_name));

                                #bn {
                                    query_type: self.query_type,
                                    tag: Conditional,
                                    bind_vars: new_bind_vars,
                                    raw_query: new_raw_query,
                                }
                            }
                        ]
                }


            })
        })
        .collect::<Vec<TokenStream2>>();

    // Struct specific methods for field based updating
    let with_qs = struct_definition
        .fields
        .into_iter()
        .map(|field| {
            let id = field_id(&field);
            let ty = &field.ty;
            let call = format!("{}", id);
            let fn_name = Ident::new(&format!("{}", id), Span::call_site());
            let bn = Ident::new(&builder_name_precursor(), Span::call_site());

            quote![
                fn #fn_name(self, value: &#ty) -> #bn<UpdateField> {
                    let mut new_bind_vars = self.bind_vars;
                    let bind_var_name = format!("withVar{}", new_bind_vars.len());
                    new_bind_vars
                        .insert(bind_var_name.clone(), serde_json::to_value(&value).unwrap());

                    let mut new_raw_query = self.raw_query;
                    new_raw_query.push(
                        format!("UPDATE item WITH {{ {}: @{} }}", #call, bind_var_name)
                    );

                    #bn {
                        query_type: self.query_type,
                        tag: UpdateField,
                        bind_vars: new_bind_vars,
                        raw_query: new_raw_query,
                    }
                }
            ]
        })
        .collect::<Vec<TokenStream2>>();

    quote![
        pub struct #builder_name<Tag: BuilderTag> {
            query_type: Option<QueryType>,
            tag: Tag,
            bind_vars: BTreeMap<String, Value>,
            raw_query: Vec<String>,
        }

        impl #builder_name<EmptyBuilder> {
            pub fn new(collection_name: &str) -> Self {
                Self {
                    query_type: None,
                    tag: EmptyBuilder,
                    bind_vars: btreemap![
                        String::from("@collection") => serde_json::to_value(&collection_name).unwrap()
                    ],
                    raw_query: vec![],
                }
            }

            pub fn create(self, elem: &#struct_name) -> #builder_name<CreateQuery> {
                let mut new_raw_query = self.raw_query;
                let mut new_bind_vars = self.bind_vars;

                new_raw_query.push(String::from("INSERT @elem"));
                new_bind_vars.insert(String::from("elem"), serde_json::to_value(&elem).unwrap());

                #builder_name {
                    query_type: Some(QueryType::Create),
                    tag: CreateQuery,
                    bind_vars: new_bind_vars,
                    raw_query: new_raw_query,
                }
            }

            pub fn read(self) -> #builder_name<ReadQuery> {
                let mut new_bind_vars = self.bind_vars;
                new_bind_vars.insert(String::from("limit"), #default_limit);

                #builder_name {
                    query_type: Some(QueryType::Read),
                    tag: ReadQuery,
                    bind_vars: new_bind_vars,
                    raw_query: Self::for_item_in_collection(),
                }
            }

            pub fn update(self) -> #builder_name<UpdateQuery> {
                #builder_name {
                    query_type: Some(QueryType::Update),
                    tag: UpdateQuery,
                    bind_vars: self.bind_vars,
                    raw_query: Self::for_item_in_collection(),
                }
            }

            pub fn delete(self) -> #builder_name<DeleteQuery> {
                #builder_name {
                    query_type: Some(QueryType::Delete),
                    tag: DeleteQuery,
                    bind_vars: self.bind_vars,
                    raw_query: Self::for_item_in_collection(),
                }
            }

            fn for_item_in_collection() -> Vec<String> {
                vec![String::from("FOR item IN @@collection")]
            }
        }

        impl<Tag: Limitable> #builder_name<Tag> {
            pub fn limit(self, limit: usize) -> #builder_name<Tag> {
                let mut new_bind_vars = self.bind_vars;
                new_bind_vars.insert(String::from("limit"), serde_json::to_value(&limit).unwrap());
                #builder_name {
                    query_type: self.query_type,
                    tag: self.tag,
                    bind_vars: new_bind_vars,
                    raw_query: self.raw_query
                }
            }
        }

        impl<Tag: Filterable> #builder_name<Tag> {
            pub fn filter(self) -> #builder_name<Filtering> {
                let mut new_raw_query = self.raw_query;
                new_raw_query.push(String::from("FILTER"));

                #builder_name {
                    query_type: self.query_type,
                    tag: Filtering,
                    bind_vars: self.bind_vars,
                    raw_query: new_raw_query,
                }
            }
        }

        impl<Tag: Conditionable> #builder_name<Tag> {
            #(#condition_qs)*
        }

        impl<Tag: UpdateWith> #builder_name<Tag> {
            pub fn replace_with(self, new_item: &#struct_name) -> #builder_name<UpdateField> {
                let mut new_bind_vars = self.bind_vars;
                let bind_var_name = format!("withVar{}", new_bind_vars.len());
                new_bind_vars
                    .insert(bind_var_name.clone(), serde_json::to_value(&new_item).unwrap());

                let mut new_raw_query = self.raw_query;
                new_raw_query.push(format!("UPDATE item WITH @{}", bind_var_name));

                #builder_name {
                    query_type: self.query_type,
                    tag: UpdateField,
                    bind_vars: new_bind_vars,
                    raw_query: new_raw_query,
                }
            }

            #(#with_qs)*
        }

        impl<Tag: LogicallyOperatable> #builder_name<Tag> {
            pub fn and(self) -> #builder_name<LogicalOperator> {
                let mut new_raw_query = self.raw_query;
                new_raw_query.push(String::from("AND"));
                #builder_name {
                    query_type: self.query_type,
                    tag: LogicalOperator,
                    bind_vars: self.bind_vars,
                    raw_query: new_raw_query
                }
            }

            pub fn or(self) -> #builder_name<LogicalOperator> {
                let mut new_raw_query = self.raw_query;
                new_raw_query.push(String::from("OR"));
                #builder_name {
                    query_type: self.query_type,
                    tag: LogicalOperator,
                    bind_vars: self.bind_vars,
                    raw_query: new_raw_query
                }
            }
        }

        impl<Tag: Buildable> #builder_name<Tag> {
            pub fn build(self) -> ArangoQuery {
                let mut new_raw_query = self.raw_query;
                let end_clause = match self.query_type {
                    Some(QueryType::Create) => "INTO @@collection",
                    Some(QueryType::Read) => "LIMIT @limit RETURN item",
                    Some(QueryType::Update) => "IN @@collection",
                    Some(QueryType::Delete) => "REMOVE item IN @@collection",
                    _ => "",
                };
                new_raw_query.push(String::from(end_clause));

                let query = new_raw_query.into_iter().map(|clause| clause + " ").collect::<String>();

                ArangoQuery::with_bind_vars(&query, self.bind_vars)
            }
        }

        pub trait #builder_factory_name {
            fn query_builder(collection_name: &str) -> #builder_name<EmptyBuilder>;
        }

        impl #builder_factory_name for #struct_name {
            fn query_builder(collection_name: &str) -> #builder_name<EmptyBuilder> {
                #builder_name::new(collection_name)
            }
        }

    ]
}

fn field_id(field: &Field) -> Ident {
    field.ident.clone().unwrap_or_else(|| {
        panic!["Derive of Arango Builder works only for structs with named fields."]
    })
}
