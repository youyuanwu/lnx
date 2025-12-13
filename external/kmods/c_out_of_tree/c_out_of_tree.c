// SPDX-License-Identifier: GPL-2.0

/*
 * C out-of-tree sample
 * Implements the same logic as the Rust out-of-tree module
 */

#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/slab.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Linux Kernel Module Contributors");
MODULE_DESCRIPTION("C out-of-tree sample");

/* Module data structure */
struct c_out_of_tree {
    int *numbers;
    size_t count;
};

static struct c_out_of_tree *mod_data;

static int __init c_out_of_tree_init(void)
{
    int ret = 0;
    
    pr_info("C out-of-tree sample (init)\n");
    
    /* Allocate module data */
    mod_data = kmalloc(sizeof(*mod_data), GFP_KERNEL);
    if (!mod_data) {
        pr_err("Failed to allocate module data\n");
        return -ENOMEM;
    }
    
    /* Allocate array for numbers */
    mod_data->count = 3;
    mod_data->numbers = kmalloc(mod_data->count * sizeof(int), GFP_KERNEL);
    if (!mod_data->numbers) {
        pr_err("Failed to allocate numbers array\n");
        kfree(mod_data);
        return -ENOMEM;
    }
    
    /* Initialize the numbers (same as Rust version) */
    mod_data->numbers[0] = 72;
    mod_data->numbers[1] = 108;
    mod_data->numbers[2] = 200;
    
    return ret;
}

static void __exit c_out_of_tree_exit(void)
{
    size_t i;
    
    if (mod_data) {
        if (mod_data->numbers) {
            /* Print the numbers */
            pr_info("My numbers are [");
            for (i = 0; i < mod_data->count; i++) {
                if (i < mod_data->count - 1)
                    pr_cont("%d, ", mod_data->numbers[i]);
                else
                    pr_cont("%d", mod_data->numbers[i]);
            }
            pr_cont("]\n");
            
            kfree(mod_data->numbers);
        }
        kfree(mod_data);
    }
    
    pr_info("C out-of-tree sample (exit)\n");
}

module_init(c_out_of_tree_init);
module_exit(c_out_of_tree_exit);
