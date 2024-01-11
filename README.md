# lockjaw
A web spider in Rust that helps to hide tracks. Its easy for most people to use a VPN or tor and hide their ip address. This code does not do that, but focues on spoofing the X-Forwarded-For, X-Originating-IP, X-Remote-IP and X-Remote-Addr headers instead. Even if an ip is spoofed, these headers can give away the user's true ip address. Most tor nodes and some VPNs will claim to strip out these headers, but if you aren't there, you don't know. The option to set a user-agent is also given to appear as a normal web browser or whatever string you wish to send to the host.

You can build this with:

```sudo make install```

Usage can be found with -h or --help:

```
$ lockjaw -h             
Lockjaw Spider 1.0
By Mephistolist
Web spider in Rust that assists with hiding tracks.

USAGE:
    lockjaw [OPTIONS] --database <DB_NAME> --url <URL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database <DB_NAME>         Sets the SQLite database name
    -s, --spoof <Spoofed_IP>         Sets the spoofed IP for headers
    -u, --url <URL>                  Sets the starting URL for the spider
    -a, --user-agent <USER_AGENT>    Sets the user agent string [default: Lockjaw Spider 1.0]
```

You can run it like this:

```
$ lockjaw -u http://localhost --spoof 127.0.0.1 --database mine.db -a "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
```

Besides the console output, this will be written to an sqlite3 database. You can view everything was found in the database specified with the following:

```
$ sqlite3 mine.db                                                                   
SQLite version 3.44.2 2023-11-24 11:41:44
Enter ".help" for usage hints.
sqlite> SELECT * FROM links;
```

You can refine the search for only urls with 200 response codes, or any response code, like this:

```select url from links where status_code = '200';```

Or just display links where <form> tags were found on the page like this:

```select url from links where has_form = 'y';```

One of the crates in the Cargo.toml warns of future deprecation on one of its dependencies. I'll have to wait for the maintainer of it to update it since I don't have access to their code:

```
warning: the following packages contain code that will be rejected by a future version of Rust: xml5ever v0.16.2 note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1
```

The laws on spoofing and spidering web forms may vary greatly country to country. If you are unsure, please refrain until you are familar with local laws. 
