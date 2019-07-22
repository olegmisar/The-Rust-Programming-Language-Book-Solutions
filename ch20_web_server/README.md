# Chapter 20. A Multithreaded Web Server

A simple multithreaded HTTP server which handles `/` path. Any other path
will result in 404 Not Found. Uses `std::net::TcpListener` to receive
requests, and `ThreadPool` to handle them simultaneously.
