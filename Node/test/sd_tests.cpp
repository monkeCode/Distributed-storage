#include<unity.h>
#include<sdCrud.h>
#include<Adafruit_GFX.h>
#include<Adafruit_SSD1306.h>

void setUp(void) {
    sdApi::begin();
}

void tearDown(void) {
    SD.end();
}

void test_avaliable(void) {
    bool res = sdApi::begin();
    TEST_ASSERT_TRUE(res);
}

void test_create_dir(void) {
    String path = "/.storage/tests/testing-dir";
    sdApi::create_dir(path);
    TEST_ASSERT_TRUE(SD.exists(path));
    File f = SD.open(path);
    TEST_ASSERT_TRUE(f.isDirectory());
    f.close();
}
void test_rm_dir(void) {
    String path = "/.storage/tests/testing-dir/";
    File f = SD.open(path + "test/file", FILE_WRITE);
    f.write("test");
    f.close();
    TEST_ASSERT_TRUE(SD.exists(path));
    sdApi::delete_file(path);
    TEST_ASSERT_FALSE(SD.exists(path));
}
void test_mv_file(void)
{
    sdApi::delete_file(".storage/tests/file");
    sdApi::delete_file(".storage/tests/newPath/tests");
    sdApi::delete_file("/megasuperpuperfile853449384");
    File f = SD.open(".storage/tests/file", FILE_WRITE);
    f.write("test");
    f.close();
    TEST_ASSERT_TRUE(sdApi::move_file(".storage/tests/file", ".storage/tests/newPath/tests"));
    TEST_ASSERT_TRUE(SD.exists(".storage/tests/newPath/tests"));
    TEST_ASSERT_TRUE(sdApi::move_file(".storage/tests/newPath/tests", "megasuperpuperfile853449384"));
    TEST_ASSERT_TRUE(SD.exists("/megasuperpuperfile853449384"));
    TEST_ASSERT_TRUE(sdApi::move_file("/megasuperpuperfile853449384", ".storage/tests/newPath/"));
    TEST_ASSERT_TRUE(SD.exists(".storage/tests/newPath/megasuperpuperfile853449384"));
    TEST_ASSERT_TRUE(sdApi::delete_file(".storage/tests/newPath/"));
}
// not needed when using generate_test_runner.rb
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_avaliable);
    RUN_TEST(test_create_dir);
    RUN_TEST(test_rm_dir);
    RUN_TEST(test_mv_file);
    return UNITY_END();
}

void setup() {
  // Wait ~2 seconds before the Unity test runner
  // establishes connection with a board Serial interface
  delay(2000);

  main();
}
void loop() {}
