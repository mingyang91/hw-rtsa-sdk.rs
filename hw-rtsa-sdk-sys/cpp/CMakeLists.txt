cmake_minimum_required(VERSION 3.25)
set(CMAKE_CXX_STANDARD 17)
project(cpp)

set(HW_RTSA_SDK ${PROJECT_SOURCE_DIR}/hw-rtsa-sdk)

include_directories(${PROJECT_SOURCE_DIR}/include ${HW_RTSA_SDK}/include)

add_library(ffi SHARED src/ffi.cpp include/ffi.h)

find_library(RTSA_CONTROL_LIBRARY NAMES rtsaControl PATHS ${HW_RTSA_SDK}/lib)


target_link_libraries(ffi PRIVATE ${RTSA_CONTROL_LIBRARY})

install(TARGETS ffi
        LIBRARY  DESTINATION lib
        ARCHIVE  DESTINATION lib/static
        RUNTIME  DESTINATION bin
        RESOURCE DESTINATION bin
        PUBLIC_HEADER DESTINATION include
)
