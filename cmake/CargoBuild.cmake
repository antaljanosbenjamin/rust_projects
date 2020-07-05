function(cargo_build_library LIB_NAME)
  set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR})

  # the OUTPUT parameter of add_custom_command doesn't support generator
  # expressions, so we have to use if-else
  if(CMAKE_BUILD_TYPE STREQUAL Release)
    set(IS_RELEASE TRUE)
    set(CARGO_BUILD_TYPE release)
  else()
    set(IS_RELEASE FALSE)
    set(CARGO_BUILD_TYPE debug)
  endif()
  set(CARGO_ARGS build --target-dir ${CARGO_TARGET_DIR}
                 $<$<BOOL:${IS_RELEASE}>:--release>)

  set(LIB_FILE
      "${CARGO_TARGET_DIR}/${CARGO_BUILD_TYPE}/${CMAKE_STATIC_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}"
  )

  file(GLOB_RECURSE LIB_SOURCES "*.rs")

  add_custom_command(
    OUTPUT ${LIB_FILE}
    COMMAND cargo ${CARGO_ARGS}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${LIB_SOURCES}
    COMMENT "running cargo")
  add_custom_target(${LIB_NAME}_target ALL DEPENDS ${LIB_FILE})
  add_library(${LIB_NAME} STATIC IMPORTED GLOBAL)
  add_dependencies(${LIB_NAME} ${LIB_NAME}_target)
  set_target_properties(${LIB_NAME} PROPERTIES IMPORTED_LOCATION ${LIB_FILE})
endfunction()
