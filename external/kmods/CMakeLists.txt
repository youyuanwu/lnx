# Get all subdirectories in the current directory
file(GLOB SUBDIRS RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/*)

# Loop through each subdirectory
foreach(SUBDIR ${SUBDIRS})
    if(IS_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/${SUBDIR})
        add_subdirectory(${SUBDIR})
        add_dependencies(rs_analyzer ${SUBDIR}_rs_analyzer)
        add_dependencies(clean_all ${SUBDIR}_clean)
    endif()
endforeach()

