project(wrappedcoordgen)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_INSTALL_LIBDIR "/lib")
add_library(wrappedcoordgen STATIC get_coordinates.cpp)
target_link_libraries(wrappedcoordgen coordgen)

install(TARGETS wrappedcoordgen
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})

