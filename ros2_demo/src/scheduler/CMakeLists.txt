project(scheduler)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 查找依赖
find_package(Threads REQUIRED)

# 调度器库
add_library(scheduler_lib
    src/task.cpp
    src/frame.cpp
    src/task_runner.cpp
    src/scheduler.cpp
)

target_include_directories(scheduler_lib PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(scheduler_lib PUBLIC
    Threads::Threads
)

# 测试
option(BUILD_TESTS "Build tests" ON)

if(BUILD_TESTS)
    # 查找Google Test
    find_package(GTest REQUIRED)
    
    # 测试可执行文件
    add_executable(scheduler_tests
        test/test_main.cpp
        test/test_advanced.cpp
    )
    
    target_link_libraries(scheduler_tests PRIVATE
        scheduler_lib
        GTest::GTest
        GTest::Main
    )
    
    # 添加测试
    enable_testing()
    add_test(NAME SchedulerTests COMMAND scheduler_tests)
endif()

# 安装
install(TARGETS scheduler_lib
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(DIRECTORY include/scheduler
    DESTINATION include
    FILES_MATCHING PATTERN "*.h"
)