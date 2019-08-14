
#include <memory>
#include <iostream>

struct Response {
    const char* body;
};

class Test {
public:
    int get() {
        this->make_input<int>();
        return 11;
    }

    int get(int i) {
        return 2 + i;
    }

    int get(int i, int j) {
        return i+j;
    }

    std::shared_ptr<Response> get(const char* o) {

        x = 10001;

        std::shared_ptr<Response> res = std::make_shared<Response>();
        res->body = o;

        return res;

    }

    template<typename T>
    void make_input();

    virtual int getok() {

        return 33;
    }

private:
    int x;
};


    template<typename T>
    void Test::make_input() {}
