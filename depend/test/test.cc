#include "test.h"
#include <iostream>

extern "C" {
    typedef class Test Test;

    Test* make_test() {
        Test* t = new Test();
        return t;
    }

    typedef void (*call) (int, Response);
    void test_get(Test* t, int path, call h) {
        std::cout << "step 1" << std::endl;

        t->ok(path);
        t->get_test(path, h);
    }

    /*
    void do_thing(int (*cb)(int)) {
        std::cout << "coming..." << std::endl;
        cb(9);
    }
    */
    void do_thing(int (*cb)(void*, int, int), void* user_data) {
        
        std::cout << "coming..." << std::endl;

        cb(user_data, 1,2);
    }


    //typedef void (*rust_callback)(Response, int32_t);
    int32_t register_callback(Response callback_target, Response::rust_callback callback) {

        //callback(callback_target, 7);


        callback_target.put(callback);

        return 1;
    }
}
