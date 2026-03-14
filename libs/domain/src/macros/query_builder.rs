#[macro_export]
macro_rules! apply_ordering {
    // ---- Single table variant ----
    (
        builder: $builder:ident,
        query: $query:expr,
        table: $table:ident,
        cols: [$( $col:ident ),* $(,)?],
        default_order: $default:ident
    ) => {
        {
            let orders = $query.parse_compact_ordering();

            if orders.is_empty() {
                $builder = $builder.order($table::$default.desc());
            } else {
                let mut first_order = true;
                for order in orders {
                    match (order.column.as_str(), order.direction.as_str()) {
                        $(
                            (stringify!($col), "asc") => {
                                if first_order {
                                    $builder = $builder.order($table::$col.asc());
                                    first_order = false;
                                } else {
                                    $builder = $builder.then_order_by($table::$col.asc());
                                }
                            },
                            (stringify!($col), "desc") => {
                                if first_order {
                                    $builder = $builder.order($table::$col.desc());
                                    first_order = false;
                                } else {
                                    $builder = $builder.then_order_by($table::$col.desc());
                                }
                            },
                        )*
                        _ => continue,
                    }
                }
            }
        }
    };

    // ---- Multi-table variant ----
    (
        builder: $builder:ident,
        query: $query:expr,
        multi_table_cols: [
            $( $table:ident => [$( $col:ident ),* $(,)?] ),* $(,)?
        ],
        default: $default_table:ident::$default_col:ident
    ) => {
        {
            let orders = $query.parse_compact_ordering();

            if orders.is_empty() {
                $builder = $builder.order($default_table::$default_col.desc());
            } else {
                let mut first_order = true;
                for order in orders {
                    match (order.column.as_str(), order.direction.as_str()) {
                        $(
                            $(
                                (concat!(stringify!($table), ".", stringify!($col)), "asc") |
                                (stringify!($col), "asc") => {
                                    if first_order {
                                        $builder = $builder.order($table::$col.asc());
                                        first_order = false;
                                    } else {
                                        $builder = $builder.then_order_by($table::$col.asc());
                                    }
                                },
                                (concat!(stringify!($table), ".", stringify!($col)), "desc") |
                                (stringify!($col), "desc") => {
                                    if first_order {
                                        $builder = $builder.order($table::$col.desc());
                                        first_order = false;
                                    } else {
                                        $builder = $builder.then_order_by($table::$col.desc());
                                    }
                                },
                            )*
                        )*
                        _ => continue,
                    }
                }
            }
        }
    };
}
