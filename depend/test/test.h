
#include <functional>

class Test {
public:
    int x;

    typedef std::function<void(int)> Handler;
    void get_test(int path, Handler h) {
        h(path);
    }

};

struct Tmp {
    void (*callback)(int32_t);
};

typedef Tmp*(__stdcall* set_callback_t)(void(*callback_t)(int32_t));
typedef void(__stdcall* use_callback_t)(Tmp*);

void callback(int32_t i) {
    printf("%d\n", i * 2);
}

//https://stackoverflow.com/questions/50188710/rust-function-that-allocates-memory-and-calls-a-c-callback-crashes
