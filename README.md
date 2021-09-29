# vertigo

List the chain of calls that end up to the given function. C codebases only.

## Options

```
[cpey@nuc vertigo]$ ./target/debug/vertigo -h
vertigo 0.1.0

USAGE:
    vertigo <search-path> --iterations <iterations> [search-functions]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --iterations <iterations>    Number of iterations (defaults to 5)

ARGS:
    <search-path>            Path of the git repo in which to run the search
    <search-functions>...    List of the function names to get the call tree for
```

## Example

```
[cpey@nuc vertigo]$ ./target/debug/vertigo -i 2 /home/cpey/repos/openssl/ SSL_ctrl
++ Call chain for SSL_ctrl
 + Function: set_protocol_version, calling: SSL_ctrl, iteration: 1
        path: test/ssltest_old.c
 + Function: SSL_clear_mode, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: SSL_set_mode, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: s_client_main, calling: SSL_set_mode, iteration: 2
        path: apps/s_client.c
 + Function: main, calling: SSL_set_mode, iteration: 2
        path: demos/bio/client-arg.c
 + Function: main, calling: SSL_set_mode, iteration: 2
        path: demos/bio/client-conf.c
 + Function: execute_test_ktls, calling: SSL_set_mode, iteration: 2
        path: test/sslapitest.c
 + Function: SSL_get_mode, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: SSL_set_mtu, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: mtu_test, calling: SSL_set_mtu, iteration: 2
        path: test/dtls_mtu_test.c
 + Function: DTLS_set_link_mtu, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: DTLS_get_link_min_mtu, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: s_client_main, calling: DTLS_get_link_min_mtu, iteration: 2
        path: apps/s_client.c
 + Function: sv_body, calling: DTLS_get_link_min_mtu, iteration: 2
        path: apps/s_server.c
 + Function: SSL_get_secure_renegotiation_support, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: print_stuff, calling: SSL_get_secure_renegotiation_support, iteration: 2
        path: apps/s_client.c
 + Function: print_connection_info, calling: SSL_get_secure_renegotiation_support, iteration: 2
        path: apps/s_server.c
 + Function: www_body, calling: SSL_get_secure_renegotiation_support, iteration: 2
        path: apps/s_server.c
 + Function: SSL_set_cert_flags, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: print_chain_flags, calling: SSL_set_cert_flags, iteration: 2
        path: apps/lib/s_cb.c
 + Function: SSL_clear_cert_flags, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: SSL_set_msg_callback_arg, calling: SSL_ctrl, iteration: 1
        path: include/openssl/ssl.h
 + Function: s_client_main, calling: SSL_set_msg_callback_arg, iteration: 2
        path: apps/s_client.c
[...]
```
