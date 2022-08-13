# wwmap
Scan a IPv4 range for a certain port.

## Usage
Write: `$ wwmap --help` for help.

    USAGE:
        wwmap [OPTIONS] --port <PORT>

    OPTIONS:
        -f, --from <FROM>
                From IPv4 - [default: 0]

        -h, --help
                Print help information

        -i, --ignore-ip-list <IGNORELIST>
                A file containing ignored IPv4 addresses (seperated by linebreaks). [default:
                ignore-ips-list.txt]

        -n, --threads <THREADS>
                Amount of threads that will be used when scanning for the specified port. [default: 1]

        -p, --port <PORT>
                Which port to scan for.

        -t, --to <TO>
                To IPv4 - [default: 4294967295]

        -v, --verbose
                Enable verbose (debug) output

        -V, --version
                Print version information
