# wwmap
Scan a IPv4 range for a certain port.

## Usage
`$ wwmap --help` for help.


	USAGE:
		wwmap [OPTIONS] <PORT> [CIDR]

	ARGS:
		<PORT>    Which port to scan for.
		<CIDR>    IPv4 subnet range (CIDR). Leave empty for the whole internet. [default: 0.0.0.0/0]

	OPTIONS:
		-h, --help
				Print help information

		-i, --ignore-ip-list <IGNORELIST>
				A file containing ignored IPv4 addresses (seperated by linebreaks). [default:
				ignore-ips-list.txt]

		-n, --threads <THREADS>
				Amount of threads that will be used when scanning for the specified port. [default: 1]

		-v, --verbose
				Enable verbose (debug) output

		-V, --version
				Print version information


### Examples
You can scan the whole internet for a HTTP server using this command: `$ wwmap 80 0.0.0.0/0`.

It is recommended to use a large amount threads for large IPv4 ranges (like the whole internet). 
