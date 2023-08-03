# brickpack-2022

## Newest version
https://github.com/afsec/brickpack-2023

## Badges
![Build Status] ![MSRV]

[Build Status]: https://github.com/afsec/brickpack/actions/workflows/makefile.yml/badge.svg
[MSRV]: https://img.shields.io/badge/rust-1.58+-brightgreen.svg?&logo=rust

## Frontend

![Frontend screenshot](/docs/img/frontend.png "Powered by Elm-ui")

## Runner

### Rendered sample code (Lua 5.1 - luajit)

![Sample code rendered screenshot ](/docs/img/sample_code_rendered.png "HTML output")


### Sample source code

![Sample code screenshot](/docs/img/sample_code.png "Lua code")


## Endpoints

- [x] `GET /` - Frontend w/Editor - Lua 5.1 support (*Under development*)


### Applets (runner) - Lua 5.1 Interpreter (modPHP-Like)

- [x] `GET /api/runner/:applet_id?param1=value&param2=value`  - query_string
- [x] `POST /api/runner/:applet_id?param1=value&param2=value`  - query_string and Form URL encoded

#### API

**Default tables:**
 - [x] `request_form` (POST)
 - [x] `request_query_string` (GET,POST)
 - [x] `request_cookies` (GET,POST)

**Functions:**
 - [x] `echo` (Render HTML like PHP)

##### Sample code
```lua
function debug_api()
    local html_header = [[
<html>
    <head>
        <style>
        table, th, td {
          border: 1px solid black;
          border-collapse: collapse;
        }
        </style>
    </head>
    <body>
    ]]
    local html_footer = [[
    </body>
</html>
]]

    echo(html_header)
    if request_form
    then
        local table_header = [[
            <h2>Logon - (request_form)</h2>
            <table>
                <thead>
                    <tr><th>Key</th><th>Value</th></tr>
                </thead><tbody>
        ]]
        echo(table_header)
    
        for k,v in pairs(request_form) do
            echo("<tr>")
            local key_column = "<td>" .. k .. "</td>"
            local value_column = "<td>" .. v .. "</td>"
            echo(key_column)
            echo(value_column)
            echo("</tr>")
        end
        echo ("</tbody></table>")
    end
    if request_query_string
    then
        local table_header = [[
            <h2>Logon - (request_query_string)</h2>
            <table>
                <thead>
                    <tr><th>Key</th><th>Value</th></tr>
                </thead><tbody>
        ]]
        echo(table_header)
    
        for k,v in pairs(request_query_string ) do
            echo("<tr>")
            local key_column = "<td>" .. k .. "</td>"
            local value_column = "<td>" .. v .. "</td>"
            echo(key_column)
            echo(value_column)
            echo("</tr>")
        end
        echo ("</tbody></table>")
        
    end
    
    if request_cookies
    then
        local table_header = [[
            <h2>Logon - (request_cookies)</h2>
            <table>
                <thead>
                    <tr><th>Key</th><th>Value</th></tr>
                </thead><tbody>
        ]]
        echo(table_header)
    
        for k,v in pairs(request_cookies) do
            echo("<tr>")
            local key_column = "<td>" .. k .. "</td>"
            local value_column = "<td>" .. v .. "</td>"
            echo(key_column)
            echo(value_column)
            echo("</tr>")
        end
        echo ("</tbody></table>")
    end

    echo(html_footer)
end


function main()
    debug_api()
end


main()
```

##### Sample code(rendered)
![Sample code screenshot](/docs/img/sample_code_api.png "Lua code")

### Applets

- [x] `HEAD /api/applets` - Read (Count)
- [x] `GET /api/applets`  - Read (All)
- [x] `GET /api/applets/:id`  - Read (One)
- [x] `POST /api/applets` - Create (One)
- [x] `PATCH /api/applets/:id` - Update (One)
- [x] `DELETE /api/applets/:id` - Delete (One)

### Users
- [x] `HEAD /api/users` - Read (Count)
- [x] `GET /api/users`  - Read (All)
- [x] `GET /api/users/:id`  - Read (One)
- [x] `POST /api/users` - Create (One)
- [x] `PATCH /api/users/:id` - Update (One)
- [x] `DELETE /api/users/:id` - Delete (One)

### Departments
- [x] `HEAD /api/departments` - Read (Count)
- [x] `GET /api/departments`  - Read (All)
- [x] `GET /api/departments/:id`  - Read (One)
- [x] `POST /api/departments` - Create (One)
- [x] `PATCH /api/departments/:id` - Update (One)
- [x] `DELETE /api/departments/:id` - Delete (One)

### Persmissions
- [x] `HEAD /api/permissions` - Read (Count)
- [x] `GET /api/permissions`  - Read (All)
- [x] `GET /api/permissions/:id`  - Read (One)
- [x] `POST /api/permissions` - Create (One)
- [x] `PATCH /api/permissions/:id` - Update (One)
- [x] `DELETE /api/permissions/:id` - Delete (One)

## Help

```
$ make

Makefile Help
======== ====

        make            - This message!
        make help       - This message!
        make prep       - Prepare project to open inside vscode
        make release    - Generate release artifact
        make debug      - Generate debug artifact
        make tests      - Compile and run tests
        make audit      - Check dependencies licenses and disclosured vulnerabilities
        make clean      - Clean compilation files and artifact folder: './dist'

   If you don't know what to choose, type:

        make release

```

## CLI help
```sh
$ ./brickpack-2022 --help
BrickPack 0.1.0



USAGE:
    brickpack-2022 [OPTIONS]

OPTIONS:
        --auto-tls-for-hostname=<HOSTNAME>                                                                                             
            Hostname for auto-generated TLS cert [env: AUTO_TLS_FOR_HOSTNAME=]

    -e, --endpoints
            Show endpoint names

        --enable-tokio-console
            Enable tokio-console [env: ENABLE_TOKIO_CONSOLE=]

    -h, --help
            Print help information

        --ipv4-address=<IPV4_ADDRESS>
            IPv4 address to listen [env: IPV4_ADDRESS=]

        --ipv4-port=<IPV4_PORT>
            Port number to listen [env: IPV4_PORT=]

        --tls-cert-path=<CERT_PATH>
            TLS certificate file [env: TLS_CERT_PATH=]

        --tls-key-path=<KEY_PATH>
            TLS private key file [env: TLS_KEY_PATH=]

    -V, --version
            Print version information

```


## Getting started

**Install dependencies:**
```sh
# Ubuntu Linux
sudo apt-get install build-essential make musl-tools -y
```

**Install [Rust](https://www.rust-lang.org/en-US/install.html):**
```sh
# Install Rust compiler
$ curl https://sh.rustup.rs -sSf | sh
.
.
.
# Clone repo
$ git clone --depth=1 --recursive https://gitlab.com/brickpack/brickpack
# Build project(Release) and Run
$ cd ./brickpack
$ make release
$ cd ./dist/release
$ ./brickpack-2022
```

## Lean artifact (< 15 MB)
> The whole artifact is built with static compiling using **musl** target.
```sh
$ ls -lh
total 13M
-rwxrwxr-x 1 user user 13M fev 11 17:06 brickpack-2022
-rw-r--r-- 1 user user 92K fev 11 23:26 database.sqlite3

$ ldd ./dist/release/brickpack-2022 
        statically linked
```

## Startup message
```sh
$ ./brickpack-2022 
2022-02-12T02:27:20.688205Z  INFO web_server: Starting App [Brickpack v0.1.0]:
2022-02-12T02:27:20.688284Z  INFO web_server: Tracing started successfully
2022-02-12T02:27:20.688309Z  INFO web_server: RUST_LOG was not set. Setting default value: RUST_LOG=info
2022-02-12T02:27:20.688356Z  INFO web_server: File `.env` not found!
2022-02-12T02:27:20.689855Z  INFO application_models::database: Bootstraping database...
2022-02-12T02:27:20.691606Z  INFO application_models::database: Database bootstrapped!
2022-02-12T02:27:20.691987Z  INFO web_server: Webserver started!
2022-02-12T02:27:20.692005Z  INFO web_server: Listening on http://127.0.0.1:8000
```

## Show endpoints
```sh
$ ./tide-crud-users -e

  Internal Endpoints:
    /                - index_page
    /maintenance     - maintenance
    /auth            - check_auth
  
  Endpoints:

   # TODO
```


## Running with all options
```sh
RUST_LOG=trace ./brickpack-2022 --ipv4-address=127.0.0.1 --ipv4-port=8000 --enable-tokio-console  --tls-cert-path ./some-test-ca/ecdsa/end.cert --tls-key-path ./some-test-ca/ecdsa/end.key
```

