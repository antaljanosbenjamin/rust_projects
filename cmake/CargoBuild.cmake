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

  set(STATIC_LIB_FILE
      "${CARGO_TARGET_DIR}/${CARGO_BUILD_TYPE}/${CMAKE_STATIC_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}"
  )
  set(SHARED_LIB_FILE
      "${CARGO_TARGET_DIR}/${CARGO_BUILD_TYPE}/${CMAKE_SHARED_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}"
  )
  set(LIB_FILES "${STATIC_LIB_FILE} ${SHARED_LIB_FILE}")

  file(GLOB_RECURSE LIB_SOURCES "*.rs")

  add_custom_command(
    OUTPUT ${LIB_FILES}
    COMMAND cargo ${CARGO_ARGS}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${LIB_SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    COMMENT "running cargo")

  add_custom_target(${LIB_NAME}_target ALL DEPENDS ${LIB_FILES})

  add_library(${LIB_NAME}_static STATIC IMPORTED GLOBAL)
  add_dependencies(${LIB_NAME}_static ${LIB_NAME}_target)
  set_target_properties(${LIB_NAME}_static PROPERTIES IMPORTED_LOCATION
                                                      ${STATIC_LIB_FILE})

  add_library(${LIB_NAME}_shared STATIC IMPORTED GLOBAL)
  add_dependencies(${LIB_NAME}_shared ${LIB_NAME}_target)
  set_target_properties(${LIB_NAME}_shared PROPERTIES IMPORTED_LOCATION
                                                      ${SHARED_LIB_FILE})
endfunction()
