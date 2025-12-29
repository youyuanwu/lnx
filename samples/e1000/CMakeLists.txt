add_custom_target(e1000_for_linux ALL
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} KERNEL_BUILD=${KBIN_ROOT}
  COMMAND ${CMAKE_COMMAND} -E copy 
    ${CMAKE_BINARY_DIR}/samples/e1000/build/e1000_for_linux.ko
    ${CMAKE_BINARY_DIR}/bin_kmods/e1000_for_linux.ko
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  COMMENT "Building and copying e1000_for_linux.ko to bin_kmods"
)

add_custom_target(e1000_for_linux_clean
  COMMAND $(MAKE) -C ${CMAKE_CURRENT_SOURCE_DIR} clean KERNEL_BUILD=${KBIN_ROOT}
  COMMAND cargo clean -p e1000
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_dependencies(clean_all
  e1000_for_linux_clean
)
