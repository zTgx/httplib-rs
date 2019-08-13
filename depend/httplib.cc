#include "cpp-httplib/httplib.h"
#include <memory>

#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-pragmas"
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif

extern "C" {
    std::shared_ptr<httplib::Client> make_client(const char * host, int port = 80, time_t timeout_sec = 300) {
        std::shared_ptr<httplib::Client> client = std::make_shared<httplib::Client>( host, port, timeout_sec );
        return client;
    }
}

#if defined(__clang__)
#pragma clang diagnostic pop
#pragma clang diagnostic pop
#endif
