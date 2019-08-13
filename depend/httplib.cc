#include "cpp-httplib/httplib.h"
#include <string>

extern "C" {
    Client* create_client(std::string host, int port = 80, time_t timeout_sec = 300) {
        // Client *client = new(Client(host, port));
        std::shared<Client> client = std::make_shared( Client(host, port) );

        return client;
    }
}
