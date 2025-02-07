cmake_minimum_required(VERSION 3.20.0)

if (NOT TEST_NOT_MAIN_BUILD)
    find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
    project(NONE)

    # NORDIC SDK APP START
    target_sources(app PRIVATE
            src/main.cpp
            src/transceiver.cpp
            src/tp25.cpp
            src/tp25_responses.cpp
    )

    # NORDIC SDK APP END
    zephyr_library_include_directories(.)
else ()
    enable_language(C CXX)
    include(FetchContent)
    FetchContent_Declare(
            googletest
            GIT_REPOSITORY https://github.com/google/googletest.git
            GIT_TAG release-1.12.1
    )

    # For Windows: Prevent overriding the parent project's compiler/linker settings
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)

    enable_testing()

    add_executable(
            tp25_test

            src/tp25_responses.cpp

            test/main.cpp

            test/responses/temp_report.cpp
    )

    target_link_libraries(
            tp25_test
            GTest::gtest_main
            gmock
    )

    #if (WIN32 AND UT_MEM_LEAK_CHECK)
    #    add_compile_definitions(tmt_test UT_MEM_LEAK_CHECK)
    #endif()

    include(GoogleTest)
    gtest_discover_tests(tp25_test)

endif ()
