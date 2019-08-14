#include "test.h"

extern "C" {
    
    typedef class Test Test;
    typedef struct Response Response;

    Test* make_test() {
        Test* t = new Test();

        return t;
    }

    int make_get(Test* t, int i, int j) {
        return t->get(i, j);
    }

    const char* make_get_char(Test* t, const char* z) {

        std::shared_ptr<Response> res = t->get(z);
        std::cout << "body content: " << res->body << std::endl;

        return "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    }
}
