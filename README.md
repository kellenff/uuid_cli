# uuid_cli

A command line utility for genrating UUIDs

## Installation

```
cargo install uuid_cli
# or
git clone https://github.com/rakenodiax/uuid_cli.git && uuid_cli && cargo install --path .
```

## Usage

```
uuid
# prints 0a781908-9fad-4b4b-a558-d9f19513bfc2
uuid -c 2
# prints 2 UUIDs
uuid --format=json
# prints as a JSON array
uuid --upper
# prints 5F8505A9-7D0A-40A0-9CAF-BADCA9C17D84
```

## Contributing

This project observes the Contributor Covenant. See [code-of-conduct.txt](code-of-conduct.txt) for more info.
