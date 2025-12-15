add_custom_target(einux_kmod ALL
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} KERNEL_BUILD=${KBIN_ROOT}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_custom_target(einux_kmod_clean
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} clean KERNEL_BUILD=${KBIN_ROOT}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_dependencies(clean_all
  einux_kmod_clean
)
