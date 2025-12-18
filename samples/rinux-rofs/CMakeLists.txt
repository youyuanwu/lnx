add_custom_target(rinux_rofs ALL
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} KERNEL_BUILD=${KBIN_ROOT}
  COMMAND ${CMAKE_COMMAND} -E copy 
    ${CMAKE_BINARY_DIR}/samples/rinux-rofs/build/rinux_rofs.ko
    ${CMAKE_BINARY_DIR}/bin_kmods/rinux_rofs.ko
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  COMMENT "Building and copying rinux_rofs.ko to bin_kmods"
)

add_custom_target(rinux_rofs_clean
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} clean KERNEL_BUILD=${KBIN_ROOT}
  COMMAND cargo clean -p rinux-rofs
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_dependencies(clean_all
  rinux_rofs_clean
)
