# rust_echo_server
A simple rust echo server.

You might want to benchmark this server with https://github.com/haraldh/rust_echo_bench

In comparison to the go echo server found at https://gist.github.com/idada/9342414 and the java netty server at https://github.com/ctron/netty-echo-server it performs nicely on my 4 Dual Intel(R) Core(TM) i7-6820HQ CPU @ 2.70GHz.

Rust:
```
$ ./echo_bench --address "127.0.0.1:12345" --number 1000 --duration 60
Benchmarking: 127.0.0.1:12345
1000 clients, running 26 bytes, 60 sec.

Speed: 697627 request/sec, 697627 response/sec
Requests: 41857626
Responses: 41857620
```

Go:
```
$ ./echo_bench --address "127.0.0.1:12345" --number 1000 --duration 60
Benchmarking: 127.0.0.1:12345
1000 clients, running 26 bytes, 60 sec.

Speed: 285356 request/sec, 285356 response/sec
Requests: 17121404
Responses: 17121401
```

Java:
```
$ ./echo_bench --address "127.0.0.1:12345" --number 1000 --duration 60
Benchmarking: 127.0.0.1:12345
1000 clients, running 26 bytes, 60 sec.

Speed: 338899 request/sec, 338899 response/sec
Requests: 20333968
Responses: 20333967
```
