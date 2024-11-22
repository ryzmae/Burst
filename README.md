**This project is no longer maintained!**

# Burst

Lighting fast in-memory database with on disk storage.

## Features

- In-memory database
- On disk storage
- Fast

## Installation

**Note**: You need to have [Rust](https://www.rust-lang.org/tools/install) installed on you're Machine.

This approach to install Burst is a Dev version in the near feature we will add support for Package Manager's like HomeBrew etc.

1. Clone the repository

```bash
git clone https://github.com/ryzmae/burst.git
```

2. Install the dependencies

```bash
cargo install --path .

```


3. Run the application

**Note**: This will compile the code and start up Burst 

```bash
make build
```

## Usage

```bash
telnet 127.0.0.1 YOUR_PORT
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Background Task

This current version of Burst is currently early stage Development version and we have not implemented a working Background Task that will keep server running in the Background

Until we add a working Background Task you use **PM2** to run the Binary.

```sh
pm2 start path/to/binaryofBurst/    
```

You find the Binary in the Target folder.

Now youre setup and you can start working with Burst.
