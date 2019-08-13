#include "cpp-httplib/httplib.h"
#include <memory>

extern "C" {
    std::shared_ptr<httplib::Client> create_client(const char * host, int port = 80, time_t timeout_sec = 300) {
        // Client *client = new(Client(host, port));
        std::shared_ptr<httplib::Client> client = std::make_shared<httplib::Client>( host, port, timeout_sec );

        return client;
    }
}
