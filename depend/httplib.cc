#include "cpp-httplib/httplib.h"
#include <iostream>

#include <ctime> //time_t

#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-pragmas"
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif

#define DEFARG(name, defval) ((#name[0]) ? (name + 0) : defval)

//Server
extern "C" {
    typedef httplib::Server Server;

    Server* make_server() {
        Server* s = new Server();
        return s;
    }

    bool listen_with(Server* s, const char *host, int port, int socket_flags) {
        std::cout << "host: " << host << "   " << "port : " << port << std::endl;
        bool r = s->listen("localhost", port, socket_flags);
        std::cout << "listen : " << r << std::endl;

        return r;
    }

    bool valid(Server* s) {
        return s->is_valid();
    }

    typedef void(*GetBack)(const httplib::Request &, httplib::Response &);
    void getx(Server* s, const char* pattern, GetBack handler) { //int (*cb)(void*, int, int), void* user_data) {

        std::cout << "path: " << pattern << std::endl;
        std::cout << "coming...: " << s->is_valid() <<" handler: "<< &handler << std::endl;

        //s->Get("/", [](const httplib::Request & /*req*/, httplib::Response &res) {
        //    res.set_content("Hello World! Rust client...\n", "text/plain");
        //});
        
        s->Get("/", handler);
    }
}

//Client
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
        std::cout << "host: " << host << std::endl;
        Client *c = new Client(host, port, timeout_sec);
        return c;
    }

    const char* get_with_path(Client* client, const char* path) {
        std::cout << "get with path: " << path << std::endl;
        std::shared_ptr<Response> res = client->Get(path);
        std::cout << "body : " << res->body << std::endl;

        return res->body.c_str();
    }
}

#if defined(__clang__)
#pragma clang diagnostic pop
#pragma clang diagnostic pop
#endif
