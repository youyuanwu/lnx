include(${CMAKE_SOURCE_DIR}/cmake/KMod.cmake)
add_kmod(
  NAME e1000_for_linux
  KDIR ${KDIR_ROOT}
)

# add_custom_command(TARGET e1000_for_linux PRE_BUILD
#   COMMAND cargo build
#   COMMAND cp ${CMAKE_SOURCE_DIR}/target/debug/deps/e1000_driver-*.o ${CMAKE_CURRENT_SOURCE_DIR}/e1000_driver.o
#   WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
# )