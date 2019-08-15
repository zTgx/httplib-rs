#include "test.h"
#include <iostream>

extern "C" {
    typedef class Test Test;

    Test* make_test() {
        Test* t = new Test();
        return t;
    }

    void get(Test* t, int path, Test::Handler h) {
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

}
