use std::{
    any,
    hash::{DefaultHasher, Hash, Hasher as _},
};

#[derive(Eq, PartialEq)]
pub(crate) struct LayoutId(pub(crate) u64);

pub(crate) fn layout_id<T>() -> LayoutId {
    let mut s = DefaultHasher::new();

    any::type_name::<T>().hash(&mut s);
    align_of::<T>().hash(&mut s);
    size_of::<T>().hash(&mut s);
    rustc_version_runtime::version().hash(&mut s);

    LayoutId(s.finish())
}
