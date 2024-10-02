# Ollama API

## Usage

### Install Cargo (Linux/MacOs)

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Install Cargo (Windows)
Windows is not advised to use as an operating system for running the api as it is deeply flawed and not ready for production(or anything), however you can do it at your own risk and find the cargo installation guide online.

### Clone the Repository

```bash
git clone https://git.staszic.waw.pl/extra/ollama-api -b dev
cd ollama-api
cargo build --release
```

### Running the Application

```bash
cargo run launch
```

### Configuring `.env`

Add your token to the headers when making requests to the API.

### Generate a New Token

```bash
cargo run new
```
This will give you a new token.

### Using the Binary

The binary is located at `target/release/llm-api` after building.
Instead of 
```bash
cargo run <command>
```
you can do
```bash
./target/release/llm-api <command>
```
you can make an alias or move it somewhere for convenience