pub trait NoneIfEmpty {
    fn none_if_empty(&self) -> Option<String>;
}

impl NoneIfEmpty for Option<String> {
    fn none_if_empty(&self) -> Option<String> {
        self.clone().filter(|val| !val.is_empty())
    }
}
