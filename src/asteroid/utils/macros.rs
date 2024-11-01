#[macro_export]
macro_rules! get {
    ($var:ident, $query:expr, $entity:expr) => {
        let Ok($var) = $query.get($entity) else {
            return;
        };
    };
}

#[macro_export]
macro_rules! get_mut {
    ($var:ident, $query:expr, $entity:expr) => {
        let Ok(mut $var) = $query.get_mut($entity) else {
            return;
        };
    };
}

#[macro_export]
macro_rules! get_single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! asset {
    ($assets:expr, $handle:expr) => {
        $assets.get($handle).expect("Cannot find asset")
    };
}
