pub unsafe fn u8_slice_from_any<T, F>(in_slice: &[T], mut cb: F)
where
    F: FnMut(&[u8]),
{
    let slice = core::slice::from_raw_parts(
        in_slice.as_ptr() as *const u8,
        in_slice.len() * core::mem::size_of::<T>(),
    );
    cb(slice);
}
