# torch

A compiler for a high level programming language called "torch" which simplifies programming Minecraft redstone CPUs.

## Features

- Configurable Code Emission
- Detailed Error Reporting

## Quick Start

```bash
git pull https://github.com/Jan1902/torch-compiler.git
cd torch-compiler
cargo run .\example-fibonacci.tch
```

## Structure

- Linker
- Lexer
- Parser
- Resolver
- Binder
- Allocator
- Emitter

## Syntax

```rust
# Simple Example

!include "io.tch"

let a = 5;
let b = 3;

let c = add(a, b);
io.output(c);

fn add(a, b)
{
    return a + b;
}
```

## Documentation

See [docs/](./docs/) for detailed documentation.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request.

## License

MIT License

## Support

For issues and questions, please open a GitHub issue.