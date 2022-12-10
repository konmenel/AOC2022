# AOC2022

## Create new day from template
For a given day "XX" do one of the following
### Rust
Run the powershell script
```console
$ .\rsetup.ps1 XX
```
or the bash script
```console
$ ./rsetup.sh XX
```

### Python
Copy the template
```console
$ cp template.py ./src/dayXX.py
```

## Run a solution
For a given day "XX" do one of the following
### Rust scripts
To run with example input:

```console
$ cargo run --bin dayXX
```

or to run with puzzle input:

```console
$ cargo run --bin dayXX --release
```

### Python scripts
No virtual environment needed (no dependencies) just python 3. To run:
```console
$ python3 src/dayXX.py
```
