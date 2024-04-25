pub mod filters;
pub mod maps;
pub mod transforms;
pub mod utils;

// Ana koleksiyon makrosu
#[macro_export]
macro_rules! collect {
    // Makro parametreleri (isteğe bağlı varsayılan koleksiyon türü ve kapasite)
    (@inner $type:ident, $capacity:expr, $($tokens:tt)*) => {
        collect!($type, $capacity, $($tokens)*)
    };
    (@inner $type:ident, $($tokens:tt)*) => {
        collect!($type, $($tokens)*)
    };
    ($($tokens:tt)*) => {
        collect!(@inner Vec, $($tokens)*)
    };

    // Sayısal aralıklarla koleksiyon oluşturma (özelleştirilebilir kapasite)
    ($type:ident, $capacity:expr, $start:expr, $end:expr) => {
        collect!($type, $capacity, $start, $end, 1)
    };
    ($type:ident, $capacity:expr, $start:expr, $end:expr, $step:expr) => {{
        let mut collection = $type::with_capacity($capacity);
        for __macro_i in ($start..=$end).step_by($step) {
            collection.push(__macro_i);
        }
        collection
    }};

    // Yineleyicilerle koleksiyon oluşturma
    ($type:ident, $iter:expr) => {{
        let iter = $iter.into_iter();
        let mut collection = $type::new();
        for __macro_i in iter {
            collection.push(__macro_i);
        }
        collection
    }};

    // Başlangıç değeri ve boyut ile koleksiyon oluşturma
    ($type:ident, $size:expr, $init:expr) => {{
        let mut collection = $type::with_capacity($size);
        for __macro_i in 0..$size {
            collection.push($init);
        }
        collection
    }};

    // Kapatma fonksiyonu ile koleksiyon oluşturma (hijyenik)
    ($type:ident, $size:expr, |$idx:ident| $expr:expr) => {{
        let mut collection = $type::with_capacity($size);
        for __macro_idx in 0..$size {
            collection.push($expr);
        }
        collection
    }};

    // Koşullu koleksiyon oluşturma
    (if $cond:expr; $type:ident, $start:expr, $end:expr) => {
        if $cond {
            collect!($type, $start, $end)
        } else {
            $type::new()
        }
    };
    (if $cond:expr; $type:ident, $iter:expr) => {
        if $cond {
            collect!($type, $iter)
        } else {
            $type::new()
        }
    };

    // İç içe koleksiyonlar oluşturma
    ($type:ident, $($inner:tt),+) => {{
        let mut collection = $type::new();
        $(
            collection.extend(collect!($inner));
        )+
        collection
    }};

    // Harita benzeri işlevsellik
    (map $type:ident, $iter:expr, |$item:ident| $expr:expr) => {
        maps::map_collect!($type, $iter, |$item| $expr)
    };

    // Filtreleme
    (filter $type:ident, $iter:expr, $cond:expr) => {
        filters::filter_collect!($type, $iter, $cond)
    };

    // Hata işleme
    (try $type:ident, $iter:expr, |$item:ident| $expr:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            match $expr {
                Ok(value) => collection.push(value),
                Err(err) => return Err(err),
            }
        }
        Ok(collection)
    }};

    // Özelleştirilebilir hata işleme (fonksiyon veya kapatma)
    (try $type:ident, $iter:expr, |$item:ident| $expr:expr, on_error = $error_handler:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            match $expr {
                Ok(value) => collection.push(value),
                Err(err) => $error_handler(err),
            }
        }
        collection
    }};

    // Özelleştirilebilir hata işleme (hata türü ve kapatma)
    (try $type:ident, $iter:expr, |$item:ident| $expr:expr, on $error_type:ty = |$err:ident| $error_expr:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            match $expr {
                Ok(value) => collection.push(value),
                Err($err) if $err.is::<$error_type>() => $error_expr,
                Err(err) => return Err(err),
            }
        }
        collection
    }};

    // Hata işleme (özelleştirilmiş hata işleyicileri ve seçenekler)
    (try $type:ident, $iter:expr, |$item:ident| $expr:expr,
        $(on $error_type:ty = |$err:ident| $error_expr:expr),*
        $(,)?
    ) => {{
        let mut collection = $type::new();
        let mut errors = Vec::new();
        for $item in $iter {
            match $expr {
                Ok(value) => collection.push(value),
                Err($err) if $( $err.is::<$error_type>() => $error_expr, )*
                Err(err) => errors.push(err), // Toplanan hatalar
            }
        }
        if errors.is_empty() {
            Ok(collection)
        } else {
            Err(errors)
        }
    }};

    // Hata işleme (koleksiyon oluşturmayı durdurma)
    (try_stop $type:ident, $iter:expr, |$item:ident| $expr:expr,
        $(on $error_type:ty = |$err:ident| $error_expr:expr),*
        $(,)?
    ) => {{
        let mut collection = $type::new();
        for $item in $iter {
            match $expr {
                Ok(value) => collection.push(value),
                Err($err) if $( $err.is::<$error_type>() => $error_expr, )*
                Err(err) => return Err(err), // Hata durumunda dur
            }
        }
        Ok(collection)
    }};

    // Asenkron koleksiyon oluşturma
    (async $type:ident, $iter:expr, |$item:ident| $expr:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            let value = $expr.await;
            collection.push(value);
        }
        collection
    }};

    // Sıralı koleksiyonlar için destek
    (sorted $type:ident, $iter:expr) => {{
        let mut collection = $type::new();
        for item in $iter {
            collection.insert(item);
        }
        collection
    }};

    // Sıralı koleksiyonlar (harita benzeri işlevsellik)
    (sorted $type:ident, $iter:expr, |$item:ident| $expr:expr) => {{
        let mut collection = $type::new();
        for $item in $iter {
            collection.insert($expr);
        }
        collection
    }};

    // Lazy değerlendirme
    (lazy $type:ident, $iter:expr) => {{
        $iter.collect::<$type>()
    }};

    // HashSet desteği
    (HashSet<$item_type:ty>, $iter:expr) => {{
        let mut collection = std::collections::HashSet::new();
        for item in $iter {
            collection.insert(item);
        }
        collection
    }};

    // HashMap desteği
    (HashMap<$key_type:ty, $value_type:ty>, $iter:expr) => {{
        let mut collection = std::collections::HashMap::new();
        for (key, value) in $iter {
            collection.insert(key, value);
        }
        collection
    }};

    // LinkedList desteği
    (LinkedList<$item_type:ty>, $iter:expr) => {{
        let mut collection = std::collections::LinkedList::new();
        for item in $iter {
            collection.push_back(item);
        }
        collection
    }};

    // Özel koleksiyon türleri için destek
    ($type:ty, $iter:expr) => {{
        $iter.collect::<$type>()
    }};

    // Reduce (reduce makrosunu kullanarak)
    (reduce $type:ident, $iter:expr, $initial:expr, |$acc:ident, $item:ident| $expr:expr) => {
        transforms::reduce_collect!($type, $iter, $initial, |$acc, $item| $expr)
    };

    // Take işlemi (take makrosunu kullanarak)
    (take $type:ident, $iter:expr, $n:expr) => {
        transforms::take_collect!($type, $iter, $n)
    };

    // Skip işlemi (skip makrosunu kullanarak)
    (skip $type:ident, $iter:expr, $n:expr) => {
        transforms::skip_collect!($type, $iter, $n)
    };

    // Zip işlemi (zip makrosunu kullanarak)
    (zip $type:ident, $iter1:expr, $iter2:expr) => {
        transforms::zip_collect!($type, $iter1, $iter2)
    };

    // Kullanıcı tanımlı fonksiyon ile haritalama
    ($type:ident, $iter:expr, $func:path) => {{
        let mut collection = $type::new();
        for item in $iter {
            collection.push($func(item));
        }
        collection
    }};

    // Sort işlemi (sort makrosunu kullanarak)
    (sort $type:ident, $iter:expr) => {
        transforms::sort_collect!($type, $iter)
    };

    // Sort_by işlemi (sort_by makrosunu kullanarak)
    (sort_by $type:ident, $iter:expr, |$a:ident, $b:ident| $expr:expr) => {
        transforms::sort_by_collect!($type, $iter, |$a, $b| $expr)
    };

    // Gruplama işlemi (group_by makrosunu kullanarak)
    (group_by $type:ident, $iter:expr, |$item:ident| $key_expr:expr) => {
        transforms::group_by_collect!($type, $iter, |$item| $key_expr)
    };

    // Flatten işlemi (flatten makrosunu kullanarak)
    (flatten $type:ident, $iter:expr) => {
        transforms::flatten_collect!($type, $iter)
    };

    // Partition işlemi (partition makrosunu kullanarak)
    (partition $type:ident, $iter:expr, |$item:ident| $cond:expr) => {
        transforms::partition_collect!($type, $iter, |$item| $cond)
    };

    // Chunk işlemi (chunk makrosunu kullanarak)
    (chunk $type:ident, $iter:expr, $size:expr) => {
        transforms::chunk_collect!($type, $iter, $size)
    };

    // Chain işlemi (chain makrosunu kullanarak)
    (chain $type:ident, $iter1:expr, $iter2:expr) => {
        transforms::chain_collect!($type, $iter1, $iter2)
    };

    // Cycle işlemi (cycle makrosunu kullanarak)
    (cycle $type:ident, $iter:expr) => {
        transforms::cycle_collect!($type, $iter)
    };

    // Enumerate işlemi (enumerate makrosunu kullanarak)
    (enumerate $type:ident, $iter:expr) => {
        transforms::enumerate_collect!($type, $iter)
    };

    // Filter_map işlemi (filter_map makrosunu kullanarak)
    (filter_map $type:ident, $iter:expr, |$item:ident| $expr:expr) => {
        transforms::filter_map_collect!($type, $iter, |$item| $expr)
    };
}
