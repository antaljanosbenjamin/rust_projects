include(CMakeParseArguments)

# +++++++++++++++++++++++++++
# _print_adding_message
# +++++++++++++++++++++++++++
function(_print_adding_message TARGET_NAME_AND_TYPE TARGET_SOURCE_FILES)
  set(MESSAGE_STR "Adding ${TARGET_NAME_AND_TYPE} with sources:")
  foreach(TARGET_SOURCE_FILE ${ARGN})
    set(MESSAGE_STR "${MESSAGE_STR}\n\t${TARGET_SOURCE_FILE}")
  endforeach()
  message(STATUS ${MESSAGE_STR})
endfunction()

# +++++++++++++++++++++++++++
# _print_adding_message
# +++++++++++++++++++++++++++
function(_print_produced_file FILE_TYPE FILE_PATH)
  message(STATUS "The produced ${FILE_TYPE} is going to be ${FILE_PATH}")
endfunction()

# +++++++++++++++++++++++++++
# _setup_cargo_variables
# +++++++++++++++++++++++++++
function(
  _setup_cargo_variables
  CARGO_ARGS_OUT
  CARGO_RESULT_DIR_OUT
  WITH_TESTS_OUT
  TESTS_PREFIX_OUT
  FOLDER_OUT
  TARGET_SOURCE_FILES_OUT
)
  cmake_parse_arguments(
    PARSED_ARGS
    "WITH_TESTS"
    "MANIFEST_PATH;FOLDER;TESTS_PREFIX"
    ""
    ${ARGN}
  )

  foreach(TARGET_SOURCE_FILE ${PARSED_ARGS_UNPARSED_ARGUMENTS})
    if(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/${TARGET_SOURCE_FILE})
      message(FATAL_ERROR "Cannot find source file: ${TARGET_SOURCE_FILE}")
    endif()
  endforeach()

  if(NOT PARSED_ARGS_MANIFEST_PATH)
    get_filename_component(MANIFEST_ABSOLUTE_PATH Cargo.toml ABSOLUTE)
    if(NOT EXISTS "${MANIFEST_ABSOLUTE_PATH}")
      # This is just a workaround because otherwise cmake-formatter fails to format
      set(ERROR_MESSAGE "Cargo.toml cannot be found in the current source directory!")
      set(ERROR_MESSAGE "${ERROR_MESSAGE} Please specify its path by MANIFEST_PATH argument!")
      message(FATAL_ERROR ${ERROR_MESSAGE})
    endif()
  else()
    get_filename_component(MANIFEST_ABSOLUTE_PATH ${PARSED_ARGS_MANIFEST_PATH} ABSOLUTE)
    if(NOT EXISTS "${MANIFEST_ABSOLUTE_PATH}")
      message(FATAL_ERROR "${PARSED_ARGS_MANIFEST_PATH} cannot be found!")
    endif()
  endif()

  set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR})

  # the OUTPUT parameter of add_custom_command doesn't support generator expressions, so we have to
  # use if-else

  set(IS_64 CMAKE_SIZEOF_VOID_P EQUAL 8)
  set(CARGO_ARGS --manifest-path ${MANIFEST_ABSOLUTE_PATH} --target-dir ${CARGO_TARGET_DIR})
  if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CARGO_BUILD_TYPE release)
    set(CARGO_ARGS ${CARGO_ARGS} --release)
  else()
    set(CARGO_BUILD_TYPE debug)
  endif()

  # https://forge.rust-lang.org/release/platform-support.html
  if(WIN32 AND MSVC)
    if(${IS_64})
      set(CARGO_TARGET_TRIPLE "x86_64-pc-windows-msvc")
    else()
      set(CARGO_TARGET_TRIPLE "i686-pc-windows-msvc")
    endif()
  elseif(ANDROID)
    if(ANDROID_SYSROOT_ABI STREQUAL "x86")
      set(CARGO_TARGET_TRIPLE "i686-linux-android")
    elseif(ANDROID_SYSROOT_ABI STREQUAL "x86_64")
      set(CARGO_TARGET_TRIPLE "x86_64-linux-android")
    elseif(ANDROID_SYSROOT_ABI STREQUAL "arm")
      set(CARGO_TARGET_TRIPLE "arm-linux-androideabi")
    elseif(ANDROID_SYSROOT_ABI STREQUAL "arm64")
      set(CARGO_TARGET_TRIPLE "aarch64-linux-android")
    else()
      message(FATAL_ERROR "Unsupported android platform!")
    endif()
  elseif(APPLE)
    set(CARGO_TARGET_TRIPLE "x86_64-apple-darwin")
  elseif(UNIX AND NOT WIN32) # UNIX AND WIN32 => Cygwin
    if(${IS_64})
      set(CARGO_TARGET_TRIPLE "x86_64-unknown-linux-gnu")
    else()
      set(CARGO_TARGET_TRIPLE "i686-unknown-linux-gnu")
    endif()
  else()
    message(FATAL_ERROR "Not supported platform")
  endif()

  set(CARGO_ARGS ${CARGO_ARGS} --target ${CARGO_TARGET_TRIPLE})

  set(${CARGO_ARGS_OUT} ${CARGO_ARGS} PARENT_SCOPE)
  set(${CARGO_RESULT_DIR_OUT} ${CARGO_TARGET_DIR}/${CARGO_TARGET_TRIPLE}/${CARGO_BUILD_TYPE}
      PARENT_SCOPE
  )
  set(${WITH_TESTS_OUT} ${PARSED_ARGS_WITH_TESTS} PARENT_SCOPE)

  if(PARSED_ARGS_TESTS_PREFIX)
    if(NOT ${PARSED_ARGS_WITH_TESTS})
      message(FATAL_ERROR "TESTS_PREFIX was specified as ${PARSED_ARGS_TESTS_PREFIX},"
                          " but WITH_TESTS option is not present!"
      )
    endif()
    set(${TESTS_PREFIX_OUT} ${PARSED_ARGS_TESTS_PREFIX} PARENT_SCOPE)
  else()
    set(${TESTS_PREFIX_OUT} "" PARENT_SCOPE)
  endif()

  if(PARSED_ARGS_FOLDER)
    set(${FOLDER_OUT} ${PARSED_ARGS_FOLDER} PARENT_SCOPE)
  endif()
  set(${TARGET_SOURCE_FILES_OUT} ${PARSED_ARGS_UNPARSED_ARGUMENTS} PARENT_SCOPE)
endfunction()

# +++++++++++++++++++++++++++
# cargo_add_library
# +++++++++++++++++++++++++++
function(cargo_add_library LIB_NAME)
  _setup_cargo_variables(
    CARGO_ARGS
    CARGO_RESULT_DIR
    WITH_TESTS
    TESTS_PREFIX
    FOLDER
    LIB_SOURCE_FILES
    ${ARGN}
  )

  set(TYPE "library")
  set(LIB_NAME_AND_TYPE "${LIB_NAME} ${TYPE}")
  _print_adding_message(${LIB_NAME_AND_TYPE} ${LIB_SOURCE_FILES})

  set(STATIC_LIB_FILE
      ${CARGO_RESULT_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}
  )
  set(SHARED_LIB_SONAME ${CMAKE_SHARED_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX})
  set(SHARED_LIB_FILE ${CARGO_RESULT_DIR}/${SHARED_LIB_SONAME})
  set(LIB_FILES ${STATIC_LIB_FILE} ${SHARED_LIB_FILE})

  _print_produced_file("shared ${TYPE}" ${SHARED_LIB_FILE})
  _print_produced_file("static ${TYPE}" ${STATIC_LIB_FILE})

  if(UNIX)
    set(LINKER_SONAME_ARG_NAME soname)
  elseif(APPLE)
    set(LINKER_SONAME_ARG_NAME install_name)
  endif()

  set(CARGO_LINKER_ARGS
      "-C link-arg=-Wl,-${LINKER_SONAME_ARG_NAME} -C link-arg=-Wl,${SHARED_LIB_SONAME}" VERBATIM
  )

  set(CARGO_LINKER_ARGS "-C link-arg=-Wl,-soname -C link-arg=-Wl,${SHARED_LIB_SONAME}" VERBATIM)

  set(CARGO_ENV_COMMAND ${CMAKE_COMMAND} -E env "RUSTFLAGS=${CARGO_LINKER_ARGS}")

  add_custom_command(
    OUTPUT ${LIB_FILES}
    COMMAND ${CARGO_ENV_COMMAND} cargo ARGS build ${CARGO_ARGS}
    DEPENDS ${LIB_SOURCE_FILES}
    COMMENT "running cargo for ${LIB_NAME_AND_TYPE} creating ${LIB_FILES}..."
  )

  set(LIB_COMMON_TARGET_NAME ${LIB_NAME}_target)
  add_custom_target(${LIB_COMMON_TARGET_NAME} ALL DEPENDS ${LIB_FILES})

  set(STATIC_LIB_TARGET_NAME ${LIB_NAME}_static)
  add_library(${STATIC_LIB_TARGET_NAME} STATIC IMPORTED GLOBAL)
  add_dependencies(${STATIC_LIB_TARGET_NAME} ${LIB_COMMON_TARGET_NAME})
  set_target_properties(${STATIC_LIB_TARGET_NAME} PROPERTIES IMPORTED_LOCATION ${STATIC_LIB_FILE})
  target_link_directories(${STATIC_LIB_TARGET_NAME} INTERFACE ${CARGO_RESULT_DIR})

  if(WIN32)
    set_property(
      TARGET ${STATIC_LIB_TARGET_NAME} PROPERTY INTERFACE_LINK_LIBRARIES advapi32 userenv ws2_32
    )
    set_property(TARGET ${STATIC_LIB_TARGET_NAME} PROPERTY INTERFACE_LINK_LIBRARIES_DEBUG msvcrtd)
    set_property(
      TARGET ${STATIC_LIB_TARGET_NAME} PROPERTY INTERFACE_LINK_LIBRARIES_MINSIZEREL msvcrt
    )
    set_property(
      TARGET ${STATIC_LIB_TARGET_NAME} PROPERTY INTERFACE_LINK_LIBRARIES_RELWITHDEBINFO msvcrt
    )
    set_property(TARGET ${STATIC_LIB_TARGET_NAME} PROPERTY INTERFACE_LINK_LIBRARIES_RELEASE msvcrt)
  elseif(UNIX)
    target_link_libraries(${STATIC_LIB_TARGET_NAME} INTERFACE pthread dl)
  else()
    message(FATAL_ERROR "Not supported platform")
  endif()

  set(SHARED_LIB_TARGET_NAME ${LIB_NAME}_shared)
  add_library(${SHARED_LIB_TARGET_NAME} SHARED IMPORTED GLOBAL)
  add_dependencies(${SHARED_LIB_TARGET_NAME} ${LIB_COMMON_TARGET_NAME})
  set_target_properties(${SHARED_LIB_TARGET_NAME} PROPERTIES IMPORTED_LOCATION ${SHARED_LIB_FILE})

  add_library(${LIB_NAME} INTERFACE)
  if(BUILD_SHARED_LIBS)
    target_link_libraries(${LIB_NAME} INTERFACE ${LIB_NAME}_shared)
  else()
    target_link_libraries(${LIB_NAME} INTERFACE ${LIB_NAME}_static)
  endif()

  if(${WITH_TESTS} AND BUILD_TESTING)
    message(STATUS "Adding tests for ${LIB_NAME} library")
    set(TESTS_TARGET_NAME ${TESTS_PREFIX}${LIB_NAME}_tests)
    add_test(NAME ${TESTS_TARGET_NAME} COMMAND cargo test ${CARGO_ARGS})
  endif()

  if(FOLDER)
    message(STATUS "Setting FOLDER property for target ${LIB_COMMON_TARGET_NAME} to ${FOLDER}")
    set_target_properties(${LIB_COMMON_TARGET_NAME} PROPERTIES FOLDER ${FOLDER})
  endif()
endfunction()

# +++++++++++++++++++++++++++
# _cargo_build_general
# +++++++++++++++++++++++++++
function(
  _cargo_build_general
  TARGET_NAME
  TARGET_TYPE
  TARGET_SUFFIX
  ADDITIONAL_CARGO_ARGS
)
  set(TARGET_NAME_AND_TYPE "${TARGET_NAME} ${TARGET_TYPE}")

  _setup_cargo_variables(
    CARGO_ARGS
    CARGO_RESULT_DIR
    WITH_TESTS
    TESTS_PREFIX
    FOLDER
    TARGET_SOURCE_FILES
    ${ARGN}
  )

  _print_adding_message(${TARGET_NAME_AND_TYPE} ${TARGET_SOURCE_FILES})

  if(${TARGET_TYPE} STREQUAL "executable")
    set(TARGET_CHOOSER_ARGS --bin ${TARGET_NAME})
    set(TARGET_FILE_PREFIX "")
  elseif(${TARGET_TYPE} STREQUAL "rust library")
    set(TARGET_CHOOSER_ARGS --lib)
    set(TARGET_FILE_PREFIX lib)
  else()
    message(FATAL_ERROR "Unsupported target type ${TARGET_TYPE}")
  endif()

  set(TARGET_FILE ${CARGO_RESULT_DIR}/${TARGET_FILE_PREFIX}${TARGET_NAME}${TARGET_SUFFIX})
  _print_produced_file(${TARGET_TYPE} ${TARGET_FILE})
  add_custom_command(
    OUTPUT ${TARGET_FILE}
    COMMAND cargo ARGS build ${TARGET_CHOOSER_ARGS} ${ADDITIONAL_CARGO_ARGS} ${CARGO_ARGS}
    DEPENDS ${TARGET_SOURCE_FILES}
    COMMENT "running cargo for ${TARGET_NAME_AND_TYPE} creating ${TARGET_FILE}..."
  )

  add_custom_target(${TARGET_NAME}_target ALL DEPENDS ${TARGET_FILE})

  if(${WITH_TESTS} AND BUILD_TESTING)
    message(STATUS "Adding tests for ${TARGET_NAME_AND_TYPE}")
    add_test(NAME ${TESTS_PREFIX}${TARGET_NAME}_tests COMMAND cargo test ${CARGO_ARGS}
             WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    )
  endif()

  message(STATUS "Setting FOLDER property for target ${TARGET_NAME} to ${FOLDER}")
  if(FOLDER)
    message(STATUS "Setting FOLDER property for target ${TARGET_NAME} to ${FOLDER}")
    set_target_properties(${TARGET_NAME}_target PROPERTIES FOLDER ${FOLDER})
  endif()
endfunction()

# +++++++++++++++++++++++++++
# cargo_add_executable
# +++++++++++++++++++++++++++
function(cargo_add_executable EXE_NAME)
  _cargo_build_general(
    ${EXE_NAME}
    executable
    "${CMAKE_EXECUTABLE_SUFFIX}" # the quotes are necessary, otherwise on linux it would evaluate to
                                 # "none", which mess up the arguments
    ""
    ${ARGN}
  )
endfunction()

# +++++++++++++++++++++++++++
# cargo_add_rust_library
# +++++++++++++++++++++++++++
function(cargo_add_rust_library LIB_NAME)
  _cargo_build_general(
    ${LIB_NAME}
    "rust library"
    .rlib
    ""
    ${ARGN}
  )
endfunction()
