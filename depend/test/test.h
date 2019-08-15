
#include <functional>
#include <iostream>

struct Response {
    int res;
 typedef void (*rust_callback)(Response, int32_t);

    Response(int x) {
        res = x;
    }

    void put(rust_callback c) {
        c(*this, 3333);
    }
};

class Test {
public:
    int x;

    typedef std::function<void(const int, Response)> Handler;
    void get_test(int path, Handler h) {
        std::cout << ".....ente..." << std::endl;

        //x = 22;
        Response r(34);
        h(3, r);
    }

    void ok(int path) {
        std::cout << "ok : " << path << std::endl;
    }

};

