function(add_kmod)
  cmake_parse_arguments(
    KMOD # prefix of output variables
    "" # list of names of the boolean arguments (only defined ones will be true)
    "NAME;KDIR" # list of names of mono-valued arguments
    #"SRCS;DEPS" # list of names of multi-valued arguments (output variables are lists)
    ""
    ${ARGN} # arguments of the function to parse, here we take the all original ones
  )
  if(NOT KMOD_NAME)
    message(FATAL_ERROR "Name param not found")
  endif(NOT KMOD_NAME)

  if(NOT KMOD_KDIR)
    message(FATAL_ERROR "Name param not found")
  endif(NOT KMOD_KDIR)

  add_custom_target(${KMOD_NAME} ALL
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} MO=${CMAKE_CURRENT_BINARY_DIR} LLVM=1 O=${KBIN_ROOT} modules
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )
# find . -name "*.rs" | xargs rustfmt
  add_custom_target(${KMOD_NAME}_fmt
    COMMAND find . -name \"*.rs\" -exec rustfmt {} \\\;
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )

  # copy kmod file into build dir
  add_custom_command(TARGET ${KMOD_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_BINARY_DIR}/bin_kmods
    COMMAND ${CMAKE_COMMAND} 
        -E copy_if_different ${CMAKE_CURRENT_BINARY_DIR}/${KMOD_NAME}.ko ${CMAKE_BINARY_DIR}/bin_kmods
  )

  add_custom_target(${KMOD_NAME}_clean
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} MO=${CMAKE_CURRENT_BINARY_DIR} LLVM=1 O=${KBIN_ROOT} clean
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )

  add_custom_target(${KMOD_NAME}_rs_analyzer
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} MO=${CMAKE_CURRENT_BINARY_DIR} O=${KBIN_ROOT} rust-analyzer
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )

  add_dependencies(rs_analyzer_all ${KMOD_NAME}_rs_analyzer)
  add_dependencies(clean_all ${KMOD_NAME}_clean)
  add_dependencies(fmt_all ${KMOD_NAME}_fmt)
endfunction()