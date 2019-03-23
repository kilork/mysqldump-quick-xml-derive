# mysqldump-quick-xml-derive - A derive macro to convert from mysqldump in xml format to struct using quick-xml

## Legal

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org/).

## Features

1. Parse mysqldump in xml file format.

## Usage

Add dependency to `Cargo.toml`:

```toml
[dependencies]
mysqldump-quick-xml = "0.1"

```

```rust
use mysqldump_quick_xml::MysqlDumpQuickXml;

#[derive(Debug, PartialEq, MysqlDumpQuickXml)]
struct Row {
    id: String,
    code: String,
}

fn main() {
    let xml = r##"
<?xml version="1.0"?>
<mysqldump xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<database name="db">
<table_data name="table1">
    <row>
        <field name="id">1</field>
        <field name="code">sample 1</field>
    </row>
    <row>
        <field name="id">2</field>
        <field name="code">sample 2</field>
    </row>
</table_data>
</database>
</mysqldump>
        "##;

    let rows = Row::from_str(xml);

    assert_eq!(
        rows,
        vec![
            Row {
                id: "1".into(),
                code: "sample 1".into()
            },
            Row {
                id: "2".into(),
                code: "sample 2".into()
            }
        ]
    )
}
```