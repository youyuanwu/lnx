// SPDX-License-Identifier: GPL-2.0
/*
 * Rust helper functions for inline kernel functions
 * 
 * This file contains wrappers for static inline functions so they can be
 * called from Rust. Inline functions don't generate symbols, so Rust
 * can't link to them directly.
 * 
 * Note: EXPORT_SYMBOL_GPL removed for out-of-tree module builds
 * to avoid symbol conflicts with the kernel.
 */

#include <linux/kconfig.h>
#include <linux/mm.h>
#include <linux/fs.h>
#include <linux/pagemap.h>
#include <linux/highmem.h>
#include <linux/slab.h>


// fs related helpers.

void rust_helper_folio_get(struct folio *folio)
{
	folio_get(folio);
}

void rust_helper_folio_put(struct folio *folio)
{
	folio_put(folio);
}

loff_t rust_helper_folio_pos(struct folio *folio)
{
	return folio_pos(folio);
}

size_t rust_helper_folio_size(struct folio *folio)
{
	return folio_size(folio);
}

void rust_helper_folio_mark_uptodate(struct folio *folio)
{
	folio_mark_uptodate(folio);
}

void rust_helper_folio_end_read(struct folio *folio, bool success)
{
	folio_end_read(folio, success);
}

void rust_helper_flush_dcache_folio(struct folio *folio)
{
	flush_dcache_folio(folio);
}

void *rust_helper_kmap_local_folio(struct folio *folio, size_t offset)
{
	return kmap_local_folio(folio, offset);
}

void rust_helper_kunmap_local(const void *vaddr)
{
	kunmap_local(vaddr);
}

void *rust_helper_alloc_inode_sb(struct super_block *sb,
				 struct kmem_cache *cache, gfp_t gfp)
{
	return alloc_inode_sb(sb, cache, gfp);
}

void rust_helper_i_uid_write(struct inode *inode, uid_t uid)
{
	i_uid_write(inode, uid);
}

void rust_helper_i_gid_write(struct inode *inode, gid_t gid)
{
	i_gid_write(inode, gid);
}

void rust_helper_mapping_set_large_folios(struct address_space *mapping)
{
	mapping_set_large_folios(mapping);
}

unsigned int rust_helper_MKDEV(unsigned int major, unsigned int minor)
{
	return MKDEV(major, minor);
}

struct kmem_cache *rust_helper_kmem_cache_create(const char *name, size_t size,
						  size_t align, unsigned long flags,
						  void (*ctor)(void *))
{
	return kmem_cache_create(name, size, align, flags, ctor);
}
