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
  set(LIB_DIR "${CARGO_TARGET_DIR}/${CARGO_BUILD_TYPE}")
  set(STATIC_LIB_FILE
      "${LIB_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}"
  )
  set(SHARED_LIB_SONAME
      "${CMAKE_SHARED_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")
  set(SHARED_LIB_FILE "${LIB_DIR}/${SHARED_LIB_SONAME}")
  set(LIB_FILES "${STATIC_LIB_FILE} ${SHARED_LIB_FILE}")

  file(GLOB_RECURSE LIB_SOURCES "*.rs")

  if(CMAKE_C_COMPILER_ID STREQUAL "GNU")
    set(CARGO_LINKER_ARGS
        "-C link-arg=-Wl,-soname -C link-arg=-Wl,${SHARED_LIB_SONAME}" VERBATIM)
  elseif(CMAKE_C_COMPILER_ID STREQUAL "Clang")
    set(CARGO_LINKER_ARGS
        "-C link-arg=-Wl,-install_name -C link-arg=-Wl,${SHARED_LIB_SONAME}"
        VERBATIM)
  endif()
  set(CARGO_ENV_COMMAND ${CMAKE_COMMAND} -E env
                        "RUSTFLAGS=${CARGO_LINKER_ARGS}")

  add_custom_command(
    OUTPUT ${LIB_FILES}
    COMMAND ${CARGO_ENV_COMMAND} cargo ARGS ${CARGO_ARGS}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${LIB_SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    COMMENT "running cargo")

  set(LIB_COMMON_TARGET_NAME ${LIB_NAME}_target)
  add_custom_target(${LIB_COMMON_TARGET_NAME} ALL DEPENDS ${LIB_FILES})

  set(STATIC_LIB_TARGET_NAME ${LIB_NAME}_static)
  add_library(${STATIC_LIB_TARGET_NAME} STATIC IMPORTED GLOBAL)
  add_dependencies(${STATIC_LIB_TARGET_NAME} ${LIB_COMMON_TARGET_NAME})
  set_target_properties(${STATIC_LIB_TARGET_NAME} PROPERTIES IMPORTED_LOCATION
                                                             ${STATIC_LIB_FILE})
  target_link_directories(${STATIC_LIB_TARGET_NAME} INTERFACE ${LIB_DIR})

  if(WIN32)
    message(ERROR "Not supported platform")
  elseif(UNIX)
    target_link_libraries(${STATIC_LIB_TARGET_NAME} INTERFACE pthread dl)
  else()
    message(ERROR "Not supported platform")
  endif()

  set(SHARED_LIB_TARGET_NAME ${LIB_NAME}_shared)
  add_library(${SHARED_LIB_TARGET_NAME} SHARED IMPORTED GLOBAL)
  add_dependencies(${SHARED_LIB_TARGET_NAME} ${LIB_COMMON_TARGET_NAME})
  set_target_properties(${SHARED_LIB_TARGET_NAME} PROPERTIES IMPORTED_LOCATION
                                                             ${SHARED_LIB_FILE})
endfunction()
