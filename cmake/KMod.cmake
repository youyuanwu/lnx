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
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} LLVM=1 modules
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )

  add_custom_target(${KMOD_NAME}_clean
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} LLVM=1 clean
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )

  add_custom_target(${KMOD_NAME}_rs_analyzer
    COMMAND $(MAKE) -C ${KMOD_KDIR} M=${CMAKE_CURRENT_SOURCE_DIR} rust-analyzer
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )
endfunction()