Brutus is a simple Sha-1 hash cracker. It compares the hash to each word in a `dictionary.txt` to find a match.

## Usage
Make sure that your wordlist file is located in your project's root directory.

```bash
cargo build
alias brutus="cargo run --"
```

- To view commands:
```bash
brutus --help
```

- To crack a hash:
```bash
brutus --file <TXT_FILE> --hash <40-DIGIT_HASH>
```
For example:
```bash
brutus --file dictionary.txt --hash cbfdac6008f9cab4083784cbd1874f76618d2a97
```
