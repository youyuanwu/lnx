/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust bindings wrapper for kernel filesystem APIs
 */

/* Include kconfig first to get IS_ENABLED and config macros */
#include <linux/kconfig.h>

/* Filesystem headers */
#include <linux/fs.h>