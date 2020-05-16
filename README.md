# Rust to python excel

Read excel file (xlsx) currently working on python3 using cpython binding.

## Requirements

   - python3
   - cargo
   - rust


## Build 

### Linux

run this command and build the target/release/reader.so
``` bash
cargo build --release
```

### Mac

run this command and build the target/release/

```bash
cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup

```

or add this file in ~/.cargo/config

```bash
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

```
and run

```bash
cargo build --release
```

### Windows
```bash
cargo build --release
```


## Usage

```python
import reader

print reader.excel_reader(absolute)
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)