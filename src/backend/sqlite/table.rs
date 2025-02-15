use super::*;

impl TableBuilder for SqliteQueryBuilder {
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn SqlWriter) {
        column_def.name.prepare(sql.as_writer(), self.quote());

        if let Some(column_type) = &column_def.types {
            write!(sql, " ").unwrap();
            self.prepare_column_type(column_type, sql);
        }

        let mut is_primary_key = false;
        let mut is_auto_increment = false;

        for column_spec in column_def.spec.iter() {
            if let ColumnSpec::PrimaryKey = column_spec {
                is_primary_key = true;
                continue;
            }
            if let ColumnSpec::AutoIncrement = column_spec {
                is_auto_increment = true;
                continue;
            }
            write!(sql, " ").unwrap();
            self.prepare_column_spec(column_spec, sql);
        }

        if is_primary_key {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::PrimaryKey, sql);
        }
        if is_auto_increment {
            write!(sql, " ").unwrap();
            self.prepare_column_spec(&ColumnSpec::AutoIncrement, sql);
        }
    }

    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn SqlWriter) {
        write!(
            sql,
            "{}",
            match column_type {
                ColumnType::Char(length) => match length {
                    Some(length) => format!("text({})", length),
                    None => "text".into(),
                },
                ColumnType::String(length) => match length {
                    Some(length) => format!("text({})", length),
                    None => "text".into(),
                },
                ColumnType::Text => "text".into(),
                ColumnType::TinyInteger(length) | ColumnType::TinyUnsigned(length) =>
                    match length {
                        Some(length) => format!("integer({})", length),
                        None => "integer".into(),
                    },
                ColumnType::SmallInteger(length) | ColumnType::SmallUnsigned(length) =>
                    match length {
                        Some(length) => format!("integer({})", length),
                        None => "integer".into(),
                    },
                ColumnType::Integer(length) | ColumnType::Unsigned(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::BigInteger(length) | ColumnType::BigUnsigned(length) => match length {
                    Some(length) => format!("integer({})", length),
                    None => "integer".into(),
                },
                ColumnType::Float(precision) => match precision {
                    Some(precision) => format!("real({})", precision),
                    None => "real".into(),
                },
                ColumnType::Double(precision) => match precision {
                    Some(precision) => format!("real({})", precision),
                    None => "real".into(),
                },
                ColumnType::Decimal(precision) => match precision {
                    Some((precision, scale)) => format!("real({}, {})", precision, scale),
                    None => "real".into(),
                },
                ColumnType::DateTime(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Timestamp(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::TimestampWithTimeZone(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Time(precision) => match precision {
                    Some(precision) => format!("text({})", precision),
                    None => "text".into(),
                },
                ColumnType::Date => "text".into(),
                ColumnType::Interval(_, _) => "unsupported".into(),
                ColumnType::Binary(blob_size) => match blob_size {
                    BlobSize::Blob(Some(length)) => format!("binary({})", length),
                    _ => "blob".into(),
                },
                ColumnType::VarBinary(length) => format!("binary({})", length),
                ColumnType::Boolean => "boolean".into(),
                ColumnType::Money(precision) => match precision {
                    Some((precision, scale)) => format!("integer({}, {})", precision, scale),
                    None => "integer".into(),
                },
                ColumnType::Json => "text".into(),
                ColumnType::JsonBinary => "text".into(),
                ColumnType::Uuid => "text(36)".into(),
                ColumnType::Custom(iden) => iden.to_string(),
                ColumnType::Enum { .. } => "text".into(),
                ColumnType::Array(_) => unimplemented!("Array is not available in Sqlite."),
                ColumnType::Cidr => unimplemented!("Cidr is not available in Sqlite."),
                ColumnType::Inet => unimplemented!("Inet is not available in Sqlite."),
                ColumnType::MacAddr => unimplemented!("MacAddr is not available in Sqlite."),
                ColumnType::Year(_) => unimplemented!("Year is not available in Sqlite."),
                ColumnType::Bit(_) => unimplemented!("Bit is not available in Sqlite."),
                ColumnType::VarBit(_) => unimplemented!("VarBit is not available in Sqlite."),
            }
        )
        .unwrap()
    }

    fn column_spec_auto_increment_keyword(&self) -> &str {
        "AUTOINCREMENT"
    }

    fn prepare_table_drop_opt(&self, _drop_opt: &TableDropOpt, _sql: &mut dyn SqlWriter) {
        // SQLite does not support table drop options
    }

    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn SqlWriter) {
        if alter.options.is_empty() {
            panic!("No alter option found")
        };
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(table) = &alter.table {
            self.prepare_table_ref_table_stmt(table, sql);
            write!(sql, " ").unwrap();
        }
        match &alter.options[0] {
            TableAlterOption::AddColumn(AddColumnOption {
                column,
                if_not_exists: _,
            }) => {
                write!(sql, "ADD COLUMN ").unwrap();
                self.prepare_column_def(column, sql);
            }
            TableAlterOption::ModifyColumn(_) => {
                panic!("Sqlite not support modifying table column")
            }
            TableAlterOption::RenameColumn(from_name, to_name) => {
                write!(sql, "RENAME COLUMN ").unwrap();
                from_name.prepare(sql.as_writer(), self.quote());
                write!(sql, " TO ").unwrap();
                to_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropColumn(col_name) => {
                write!(sql, "DROP COLUMN ").unwrap();
                col_name.prepare(sql.as_writer(), self.quote());
            }
            TableAlterOption::DropForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
            TableAlterOption::AddForeignKey(_) => {
                panic!("Sqlite does not support modification of foreign key constraints to existing tables");
            }
        }
    }

    fn prepare_table_rename_statement(
        &self,
        rename: &TableRenameStatement,
        sql: &mut dyn SqlWriter,
    ) {
        write!(sql, "ALTER TABLE ").unwrap();
        if let Some(from_name) = &rename.from_name {
            self.prepare_table_ref_table_stmt(from_name, sql);
        }
        write!(sql, " RENAME TO ").unwrap();
        if let Some(to_name) = &rename.to_name {
            self.prepare_table_ref_table_stmt(to_name, sql);
        }
    }
}
