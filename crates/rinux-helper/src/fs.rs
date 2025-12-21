use core::ffi::c_void;

// Link the compiled C helpers
extern "C" {
    pub fn rust_helper_folio_get(folio: *mut c_void);
    pub fn rust_helper_folio_put(folio: *mut c_void);
    pub fn rust_helper_folio_pos(folio: *mut c_void) -> i64;
    pub fn rust_helper_folio_size(folio: *mut c_void) -> usize;
    pub fn rust_helper_folio_mark_uptodate(folio: *mut c_void);
    pub fn rust_helper_folio_end_read(folio: *mut c_void, success: bool);
    pub fn rust_helper_flush_dcache_folio(folio: *mut c_void);
    pub fn rust_helper_kmap_local_folio(folio: *mut c_void, offset: usize) -> *mut c_void;
    pub fn rust_helper_kunmap_local(vaddr: *const c_void);
    pub fn rust_helper_alloc_inode_sb(sb: *mut c_void, cache: *mut c_void, gfp: u32)
        -> *mut c_void;
    pub fn rust_helper_i_uid_write(inode: *mut c_void, uid: u32);
    pub fn rust_helper_i_gid_write(inode: *mut c_void, gid: u32);
    pub fn rust_helper_mapping_set_large_folios(mapping: *mut c_void);
    pub fn rust_helper_MKDEV(major: u32, minor: u32) -> u32;
    pub fn rust_helper_kmem_cache_create(
        name: *const i8,
        size: usize,
        align: usize,
        flags: u64,
        ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut c_void;
}

// Re-export with cleaner names
#[inline]
pub unsafe fn folio_get(folio: *mut c_void) {
    unsafe { rust_helper_folio_get(folio) };
}

#[inline]
pub unsafe fn folio_put(folio: *mut c_void) {
    unsafe { rust_helper_folio_put(folio) };
}

#[inline]
pub unsafe fn folio_pos(folio: *mut c_void) -> i64 {
    unsafe { rust_helper_folio_pos(folio) }
}

#[inline]
pub unsafe fn folio_size(folio: *mut c_void) -> usize {
    unsafe { rust_helper_folio_size(folio) }
}

#[inline]
pub unsafe fn folio_mark_uptodate(folio: *mut c_void) {
    unsafe { rust_helper_folio_mark_uptodate(folio) };
}

#[inline]
pub unsafe fn folio_end_read(folio: *mut c_void, success: bool) {
    unsafe { rust_helper_folio_end_read(folio, success) };
}

#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut c_void) {
    unsafe { rust_helper_flush_dcache_folio(folio) };
}

#[inline]
pub unsafe fn kmap_local_folio(folio: *mut c_void, offset: usize) -> *mut c_void {
    unsafe { rust_helper_kmap_local_folio(folio, offset) }
}

#[inline]
pub unsafe fn kunmap_local(vaddr: *const c_void) {
    unsafe { rust_helper_kunmap_local(vaddr) };
}

#[inline]
pub unsafe fn alloc_inode_sb(sb: *mut c_void, cache: *mut c_void, gfp: u32) -> *mut c_void {
    unsafe { rust_helper_alloc_inode_sb(sb, cache, gfp) }
}

#[inline]
pub unsafe fn i_uid_write(inode: *mut c_void, uid: u32) {
    unsafe { rust_helper_i_uid_write(inode, uid) };
}

#[inline]
pub unsafe fn i_gid_write(inode: *mut c_void, gid: u32) {
    unsafe { rust_helper_i_gid_write(inode, gid) };
}

#[inline]
pub unsafe fn mapping_set_large_folios(mapping: *mut c_void) {
    unsafe { rust_helper_mapping_set_large_folios(mapping) };
}

#[inline]
pub fn mkdev(major: u32, minor: u32) -> u32 {
    unsafe { rust_helper_MKDEV(major, minor) }
}

#[inline]
pub unsafe fn kmem_cache_create(
    name: *const i8,
    size: usize,
    align: usize,
    flags: u64,
    ctor: Option<unsafe extern "C" fn(*mut c_void)>,
) -> *mut c_void {
    unsafe { rust_helper_kmem_cache_create(name, size, align, flags, ctor) }
}
