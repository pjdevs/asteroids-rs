#[macro_export]
macro_rules! get {
    ($var:ident, $query:expr, $entity:expr, $not_found:expr) => {
        let Ok($var) = $query.get($entity) else {
            $not_found;
        };
    };
}

#[macro_export]
macro_rules! get_mut {
    ($var:ident, $query:expr, $entity:expr, $not_found:expr) => {
        let Ok(mut $var) = $query.get_mut($entity) else {
            $not_found;
        };
    };
}

#[macro_export]
macro_rules! asset {
    ($assets:expr, $handle:expr) => {
        $assets.get($handle).expect("Cannot find asset")
    };
}
