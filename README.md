# Rust to python excel

Read excel file (xlsx) currently working on python3 using cpython binding.

## Requirements

   - python3
   - cargo
   - rust
   - cpython


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

Image of excel below

![Excel Images](https://github.com/marcpar/rust2py_excel_reader/blob/master/test.png)

Result

```json
{0: [{'SKU': 'MP13-0001', 'Product Designation': 'Mac book Pro 13"', 'Unit Price': '1299'}], 1: [{'SKU': 'MP16-0001', 'Product Designation': 'Mac book Pro 16"', 'Unit Price': '1599'}], 2: [{'SKU': 'IPX-0001', 'Product Designation': 'IPhone X', 'Unit Price': '1199'}]}
```

## running test

build program

```bash
python3 ./tests/test_reader.py
```

```python3
import rust2py
rust2py.excel_reader("../sample.xlsx")
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
