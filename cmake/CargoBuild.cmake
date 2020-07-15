include(CMakeParseArguments)

function(cargo_build_library LIB_NAME)
  cmake_parse_arguments(
    PARSED_ARGS
    "WITH_TESTS"
    ""
    ""
    ${ARGN}
  )

  # if(NOT PARSED_ARGS_LIB_NAME) message(FATAL_ERROR "The name of library must be specified!")
  # endif()

  # set(LIB_NAME ${PARSED_ARGS_LIB_NAME})
  set(WITH_TESTS ${PARSED_ARGS_WITH_TESTS})
  set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR})

  # the OUTPUT parameter of add_custom_command doesn't support generator expressions, so we have to
  # use if-else

  set(IS_64 CMAKE_SIZEOF_VOID_P EQUAL 8)
  set(CARGO_ARGS --target-dir ${CARGO_TARGET_DIR})
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

  set(LIB_DIR ${CARGO_TARGET_DIR}/${CARGO_TARGET_TRIPLE}/${CARGO_BUILD_TYPE})
  set(STATIC_LIB_FILE
      ${LIB_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_STATIC_LIBRARY_SUFFIX}
  )
  set(SHARED_LIB_SONAME ${CMAKE_SHARED_LIBRARY_PREFIX}${LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX})
  set(SHARED_LIB_FILE ${LIB_DIR}/${SHARED_LIB_SONAME})
  set(LIB_FILES ${STATIC_LIB_FILE} ${SHARED_LIB_FILE})

  file(GLOB_RECURSE LIB_SOURCES "*.rs")

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
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${LIB_SOURCES} ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    COMMENT "running cargo for ${LIB_NAME} creating (${LIB_FILES})..."
  )

  set(LIB_COMMON_TARGET_NAME ${LIB_NAME}_target)
  add_custom_target(${LIB_COMMON_TARGET_NAME} ALL DEPENDS ${LIB_FILES})

  set(STATIC_LIB_TARGET_NAME ${LIB_NAME}_static)
  add_library(${STATIC_LIB_TARGET_NAME} STATIC IMPORTED GLOBAL)
  add_dependencies(${STATIC_LIB_TARGET_NAME} ${LIB_COMMON_TARGET_NAME})
  set_target_properties(${STATIC_LIB_TARGET_NAME} PROPERTIES IMPORTED_LOCATION ${STATIC_LIB_FILE})
  target_link_directories(${STATIC_LIB_TARGET_NAME} INTERFACE ${LIB_DIR})

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

  if(WITH_TESTS AND BUILD_TESTING)
    add_test(NAME ${LIB_NAME}_tests COMMAND cargo test ${CARGO_ARGS})
  endif()
endfunction()
