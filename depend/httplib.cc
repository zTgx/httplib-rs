#include "cpp-httplib/httplib.h"
#include <iostream>

#include <ctime> //time_t
#include <string.h>

#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-pragmas"
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif

#define DEFARG(name, defval) ((#name[0]) ? (name + 0) : defval)

//Response
extern "C" {
    typedef httplib::Response Response;

    void r_set_redirect(Response* r, const char *uri) {
        std::cout << "redirect to uri: " << uri << std::endl;
        r->set_redirect(uri);
    }

    void r_set_content_n(Response*r, const char *s, size_t n, const char *content_type) {
        r->set_content(s, n, content_type);
    }

    void r_set_content(Response* r, const char* s, const char* content_type) {
        r->set_content(s, content_type);
    }
}

//Server
extern "C" {
    typedef httplib::Server Server;

    Server* make_server() {
        Server* s = new Server();
        return s;
    }

    bool listen_with(Server* s, const char *host, int port, int socket_flags) {
        std::cout << "host: " << host << "   " << "port : " << port << std::endl;
        bool r = s->listen(host, port, socket_flags);
        return r;
    }

    bool valid(Server* s) {
        return s->is_valid();
    }

    typedef void(*GetBack)(const httplib::Request &, httplib::Response &);
    void getx(Server* s, const char* pattern, GetBack handler) { //int (*cb)(void*, int, int), void* user_data) {

        std::cout << "path: " << pattern << std::endl;
        std::cout << "coming...: " << s->is_valid() <<" handler: "<< &handler << std::endl;
        s->Get(pattern, handler);
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
        std::cout << "connect host: " << host << ":" << port << std::endl;
        Client *c = new Client(host, port, timeout_sec);
        return c;
    }

    Response* get_with_path(Client* client, const char* pattern, int len) {
        char dst[256];
        memcpy (dst ,pattern, len);
        printf("pattern : %s\n", dst);
        std::shared_ptr<Response> res = client->Get(dst);

        printf("status: %d\n", res->status);
        printf("body : %s\n", res->body.c_str());

        return res.get();
    }
}

#if defined(__clang__)
#pragma clang diagnostic pop
#pragma clang diagnostic pop
#endif
