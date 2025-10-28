# rand_tool

A formidable random number generator capable of producing passwords,
service ports, and UUIDs, alongside the ability to encode and decode Base64.
## Installation

```bash
cargo install rand_tool
```

## Features

- **Password Generation**: Create strong, random passwords with configurable length and character sets
- **Port Generation**: Generate random port numbers within specified ranges
- **UUID Generation**: Generate Version 4 UUIDs
- **Base64 Operations**: Encode and decode Base64 strings

## Usage

### Password Generation

Generate random passwords with customizable options:

```bash
# Generate 5 passwords (18 chars, numbers + uppercase + lowercase, default)
rand_tool pwd

# Generate 10 passwords with 24 characters
rand_tool -c 10 pwd -c 24

# Generate password with special symbols
rand_tool pwd -s

# Generate password without numbers
rand_tool pwd -n

# Generate password with only lowercase letters
rand_tool pwd -u -n
```

**Options:**
- `-c, --count <COUNT>`: Number of passwords to generate (default: 5)
- `-c --length <LENGTH>`: Password length (default: 18)
- `-n --numbers`: Exclude digits 0-9
- `-u --uppercase`: Exclude uppercase letters A-Z
- `-l --lowercase`: Exclude lowercase letters a-z
- `-s, --symbols`: Include special characters

### Port Generation

Generate random port numbers within a specified range:

```bash
# Generate 5 random ports (default range: 1024-49151)
rand_tool port

# Generate 10 ports in custom range
rand_tool -c 10 port --range 8000-9000

# Generate ports in specific range
rand_tool port -r 3000-3100
```

**Options:**
- `-c, --count <COUNT>`: Number of ports to generate (default: 5)
- `-r, --range <RANGE>`: Port range in format "min-max" (default: 1024-49151)

### UUID Generation

Generate Version 4 UUIDs:

```bash
# Generate 5 UUIDs (default)
rand_tool uuid

# Generate 20 UUIDs
rand_tool -c 20 uuid
```

### Base64 Operations

Encode and decode Base64 strings:

```bash
# Encode string to Base64
rand_tool base64 --encode "Hello, World!"

# Decode Base64 string
rand_tool base64 --decode "SGVsbG8sIFdvcmxkIQ=="
```

**Options:**
- `-e, --encode <STRING>`: Encode UTF-8 string to Base64
- `-d, --decode <STRING>`: Decode Base64 string to UTF-8

## Global Options

- `-c, --count <COUNT>`: Default quantity of items to generate (default: 5)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

```bash
# Generate 3 strong passwords with symbols
rand_tool -c 3 pwd --length 32 --symbols

# Generate 100 ports for testing
rand_tool -c 100 port --range 10000-20000

# Generate single UUID
rand_tool -c 1 uuid

# Encode sensitive data
rand_tool base64 -e "my-secret-key"
```