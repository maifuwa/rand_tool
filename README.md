# rand_tool

Powerful command-line tool for generating strong, random passwords and ports.

install `cargo install rand_tool`
uninstall `cargo uninstall rand_tool`

```text
Usage: rand_tool [OPTIONS]

Options:
  -P, --port             Generates random ports instead of passwords
  -r, --range <RANGE>    Set the range of ports to generate [default: 1024-49151]
  -c, --count <COUNT>    Set the number of passwords or ports to generate [default: 5]
  -n, --number           Do not include numbers in the password
  -u, --uppercase        Do not include uppercase characters in the password
  -l, --lowercase        Do not include lowercase characters in the password
  -s, --symbols          Include special characters in the password
  -p, --spaces           Include spaces in the password
  -L, --length <LENGTH>  Set the password length [default: 18]
  -h, --help             Print help
  -V, --version          Print version
  ```