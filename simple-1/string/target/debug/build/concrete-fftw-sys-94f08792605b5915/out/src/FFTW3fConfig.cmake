# defined since 2.8.3
if (CMAKE_VERSION VERSION_LESS 2.8.3)
  get_filename_component (CMAKE_CURRENT_LIST_DIR ${CMAKE_CURRENT_LIST_FILE} PATH)
endif ()

# Allows loading FFTW3 settings from another project
set (FFTW3_CONFIG_FILE "${CMAKE_CURRENT_LIST_FILE}")

set (FFTW3f_LIBRARIES fftw3f)
set (FFTW3f_LIBRARY_DIRS /Users/mukund/fhe-exp/fhe-experiments/simple-1/string/target/debug/build/concrete-fftw-sys-94f08792605b5915/out/lib)
set (FFTW3f_INCLUDE_DIRS /Users/mukund/fhe-exp/fhe-experiments/simple-1/string/target/debug/build/concrete-fftw-sys-94f08792605b5915/out/include)

include ("${CMAKE_CURRENT_LIST_DIR}/FFTW3LibraryDepends.cmake")

if (CMAKE_VERSION VERSION_LESS 2.8.3)
  set (CMAKE_CURRENT_LIST_DIR)
endif ()
