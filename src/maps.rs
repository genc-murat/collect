// Haritalama makrosu
#[macro_export]
macro_rules! map_collect {
    ($type:ident, $iter:expr, |$item:ident| $expr:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            collection.push($expr);
        }
        collection
    }};
}
