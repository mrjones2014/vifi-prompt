pub trait NoneIfEmpty {
    fn none_if_empty(&self) -> Option<String>;
}

impl NoneIfEmpty for Option<String> {
    fn none_if_empty(&self) -> Option<String> {
        if let Some(val) = self {
            if val.is_empty() {
                None
            } else {
                Some(val.to_string())
            }
        } else {
            None
        }
    }
}
