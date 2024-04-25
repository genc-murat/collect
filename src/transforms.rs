// Reduce makrosu
#[macro_export]
macro_rules! reduce_collect {
    ($type:ident, $iter:expr, $initial:expr, |$acc:ident, $item:ident| $expr:expr) => {{
        let mut acc = $initial;
        for $item in $iter {
            acc = $expr;
        }
        acc
    }};
}

// Take işlemi makrosu
#[macro_export]
macro_rules! take_collect {
    ($type:ident, $iter:expr, $n:expr) => {{
        $iter.take($n).collect::<$type>()
    }};
}

// Skip işlemi makrosu
#[macro_export]
macro_rules! skip_collect {
    ($type:ident, $iter:expr, $n:expr) => {{
        $iter.skip($n).collect::<$type>()
    }};
}

// Zip işlemi makrosu
#[macro_export]
macro_rules! zip_collect {
    ($type:ident, $iter1:expr, $iter2:expr) => {{
        $iter1.zip($iter2).collect::<$type>()
    }};
}

// Sort işlemi makrosu
#[macro_export]
macro_rules! sort_collect {
    ($type:ident, $iter:expr) => {{
        let mut collection = $iter.collect::<$type>();
        collection.sort();
        collection
    }};
}

// Sort_by işlemi makrosu
#[macro_export]
macro_rules! sort_by_collect {
    ($type:ident, $iter:expr, |$a:ident, $b:ident| $expr:expr) => {{
        let mut collection = $iter.collect::<$type>();
        collection.sort_by(|$a, $b| $expr);
        collection
    }};
}

// Gruplama işlemi makrosu
#[macro_export]
macro_rules! group_by_collect {
    ($type:ident, $iter:expr, |$item:ident| $key_expr:expr) => {{
        let mut groups = $type::new();
        for $item in $iter {
            let key = $key_expr;
            groups.entry(key).or_insert_with($type::new).push($item);
        }
        groups
    }};
}

// Flatten işlemi makrosu
#[macro_export]
macro_rules! flatten_collect {
    ($type:ident, $iter:expr) => {{
        $iter.flatten().collect::<$type>()
    }};
}

// Partition işlemi makrosu
#[macro_export]
macro_rules! partition_collect {
    ($type:ident, $iter:expr, |$item:ident| $cond:expr) => {{
        let (left, right): ($type, $type) = $iter.partition(|$item| $cond);
        (left, right)
    }};
}

// Chunk işlemi makrosu
#[macro_export]
macro_rules! chunk_collect {
    ($type:ident, $iter:expr, $size:expr) => {{
        $iter.chunks($size).collect::<$type>()
    }};
}

// Chain işlemi makrosu
#[macro_export]
macro_rules! chain_collect {
    ($type:ident, $iter1:expr, $iter2:expr) => {{
        $iter1.chain($iter2).collect::<$type>()
    }};
}

// Cycle işlemi makrosu
#[macro_export]
macro_rules! cycle_collect {
    ($type:ident, $iter:expr) => {{
        $iter.cycle().collect::<$type>()
    }};
}

// Enumerate işlemi makrosu
#[macro_export]
macro_rules! enumerate_collect {
    ($type:ident, $iter:expr) => {{
        $iter.enumerate().collect::<$type>()
    }};
}

// Filter_map işlemi makrosu
#[macro_export]
macro_rules! filter_map_collect {
    ($type:ident, $iter:expr, |$item:ident| $expr:expr) => {{
        $iter.filter_map(|$item| $expr).collect::<$type>()
    }};
}
