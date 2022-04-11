use crate::weak::weak;

pub(crate) fn libfuzzer_runone_fn() -> unsafe extern "C" fn(*const u8, usize) -> std::os::raw::c_int
{
    #[allow(non_snake_case)]
    {
        weak!(fn LLVMFuzzerTestOneInput(*const u8, usize) -> std::os::raw::c_int);

        LLVMFuzzerTestOneInput
            .get()
            .expect("failed to get LLVMFuzzerTestOneInput")
    }
}

pub(crate) fn libfuzzer_initialize_fn() -> Option<
    unsafe extern "C" fn(*mut std::os::raw::c_int, *mut *const *const u8) -> std::os::raw::c_int,
> {
    #[allow(non_snake_case)]
    {
        weak!(fn LLVMFuzzerInitialize(*mut std::os::raw::c_int, *mut *const *const u8) -> std::os::raw::c_int);

        LLVMFuzzerInitialize.get()
    }
}

pub(crate) fn asan_unpoison_memory_region_fn(
) -> Option<unsafe extern "C" fn(*const u8, usize) -> std::os::raw::c_void> {
    weak!(fn __asan_unpoison_memory_region(*const u8, usize) -> std::os::raw::c_void);

    __asan_unpoison_memory_region.get()
}

pub(crate) fn asan_poison_memory_region_fn(
) -> Option<unsafe extern "C" fn(*const u8, usize) -> std::os::raw::c_void> {
    weak!(fn __asan_poison_memory_region(*const u8, usize) -> std::os::raw::c_void);

    __asan_poison_memory_region.get()
}

pub(crate) fn msan_unpoison_memory_region_fn(
) -> Option<unsafe extern "C" fn(*const u8, usize) -> std::os::raw::c_void> {
    weak!(fn __msan_unpoison_memory_region(*const u8, usize) -> std::os::raw::c_void);

    __msan_unpoison_memory_region.get()
}

pub(crate) fn msan_poison_memory_region_fn(
) -> Option<unsafe extern "C" fn(*const u8, usize) -> std::os::raw::c_void> {
    weak!(fn __msan_poison_memory_region(*const u8, usize) -> std::os::raw::c_void);

    __msan_poison_memory_region.get()
}
