/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust bindings wrapper for kernel filesystem APIs
 */

/* Include kconfig first to get IS_ENABLED and config macros */
#include <linux/kconfig.h>

/* Filesystem headers */
#include <linux/fs.h>
#include <linux/fs_context.h>
#include <linux/mm.h>
//#include <linux/pagemap.h>
#include <linux/slab.h>

const slab_flags_t BINDINGS_SLAB_RECLAIM_ACCOUNT = SLAB_RECLAIM_ACCOUNT;
const slab_flags_t BINDINGS_SLAB_ACCOUNT = SLAB_ACCOUNT;

// const slab_flags_t BINDINGS_SLAB_MEM_SPREAD = SLAB_MEM_SPREAD;

const loff_t BINDINGS_MAX_LFS_FILESIZE = MAX_LFS_FILESIZE;
// const size_t BINDINGS_PAGE_SIZE = PAGE_SIZE;