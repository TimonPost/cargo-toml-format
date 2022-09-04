use toml_edit::{Item, Key, Value};

pub fn item_len(item: &Item) -> usize {
    match item {
        Item::None => 0,
        Item::Value(val) => match val {
            toml_edit::Value::String(str) => return str.value().char_indices().count(),
            toml_edit::Value::Integer(int) => return int.to_repr().as_raw().char_indices().count(),
            toml_edit::Value::Float(float) => {
                return float.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Boolean(boolean) => {
                return boolean.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Datetime(datetime) => {
                return datetime.to_repr().as_raw().char_indices().count()
            }
            toml_edit::Value::Array(a) => a.iter().map(|i| item_len(&Item::Value(i.clone()))).sum(),
            toml_edit::Value::InlineTable(inline_table) => table_len(inline_table.get_values()),
        },
        Item::Table(table) => table_len(table.get_values()),
        Item::ArrayOfTables(array_table) => {
            array_table.iter().map(|t| table_len(t.get_values())).sum()
        }
    }
}

pub fn table_len(key_values: Vec<(Vec<&Key>, &Value)>) -> usize {
    key_values
        .iter()
        .map(|(keys, value)| -> usize {
            let keys_len: usize = keys
                .iter()
                .map(|key| key.get().char_indices().count())
                .sum();
            let value_len = item_len(&Item::Value((*value).clone()));
            keys_len + value_len
        })
        .sum()
}
