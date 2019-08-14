#include "cpp-httplib/httplib.h"
#include <iostream>

#include <ctime> //time_t

#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-pragmas"
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif

extern "C" {

    typedef httplib::Client Client;
    typedef httplib::Response Response;

    Client* make_client_with_host(const char* host) {
        Client *c = new Client(host);
        return c;
    }

    Client* make_client_with_host_port(const char* host, int port) {
        Client *c = new Client(host, port);
        return c;
    }

    Client* make_client_with_host_port_timeout(const char* host, int port, time_t timeout_sec) {
        Client *c = new Client(host, port, timeout_sec);
        return c;
    }

    const char* get_with_path(Client* client, const char* path) {
        std::shared_ptr<Response> res = client->Get(path);
        std::cout << "body : " << res->body << std::endl;

        return res->body.c_str();
    }
}

#if defined(__clang__)
#pragma clang diagnostic pop
#pragma clang diagnostic pop
#endif
