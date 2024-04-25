// Filtreleme makrosu
#[macro_export]
macro_rules! filter_collect {
    ($type:ident, $iter:expr, $cond:expr) => {{
        let mut collection = $type::new();
        for item in $iter {
            if $cond {
                collection.push(item);
            }
        }
        collection
    }};
}
