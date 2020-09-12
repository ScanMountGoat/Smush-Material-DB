extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;

use inflector::cases::pascalcase::to_pascal_case;
use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Ident};

fn get_sql_column_names(field_identifiers: &Vec<&Option<Ident>>) -> String {
    // Adapt rust naming conventions to match the table columns.
    // my_struct_field1, (Rust) -> MyStructField (SQL)
    let field_names: Vec<String> = field_identifiers
        .iter()
        .map(|n| match n {
            Some(e) => to_pascal_case(&e.to_string()),
            _ => "".to_owned(),
        })
        .collect();

    field_names.join(",")
}

fn get_table_name(implementing_type: &Ident, attrs: &Vec<Attribute>) -> String {
    for attr in attrs {
        if attr.path.is_ident("table") {
            let lit: syn::LitStr = attr.parse_args().unwrap();
            return lit.value();
        }
    }

    // Use the type's name if no name was specified.
    implementing_type.to_string()
}

fn get_sql_text(table_name: &str, column_names: &str, column_count: usize) -> String {
    // Construct the SQL statement to insert the record.
    let value_specifiers = vec!["?"; column_count].join(",");
    format!(
        "INSERT INTO {}({}) VALUES({})",
        table_name, column_names, value_specifiers
    )
}

#[proc_macro_derive(SqlInsert, attributes(table))]
pub fn sql_insert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let implementing_type = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_identifiers: Vec<&Option<Ident>> = fields.iter().map(|field| &field.ident).collect();
    let column_names = get_sql_column_names(&field_identifiers);
    let table_name = get_table_name(implementing_type, &input.attrs);
    let sql_text = get_sql_text(&table_name, &column_names, field_identifiers.len());

    // Create the trait implementation.
    let expanded = quote! {
        impl crate::records::SqlInsert for #implementing_type {
            fn insert(&self, transaction: &mut Transaction) -> Result<()> {
                transaction
                    .prepare_cached(
                        #sql_text,
                    )?
                    .execute(params![
                        #(
                            self.#field_identifiers,
                        )*
                    ])?;
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
