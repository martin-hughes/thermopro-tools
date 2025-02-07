#include "gtest/gtest.h"

/*#ifdef UT_MEM_LEAK_CHECK
#include "infra/win_mem_leak.h"
#endif*/

using namespace std;

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
/*
#ifdef UT_MEM_LEAK_CHECK
  ::testing::TestEventListeners& listeners = ::testing::UnitTest::GetInstance()->listeners();
  listeners.Append(new MemLeakListener);
#endif
*/
  return RUN_ALL_TESTS();
}
