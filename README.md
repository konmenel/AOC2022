# AOC2022
My solutions to Advent of Code 2022.

## To create new day from template
For a given day "**XX**" do one of the following

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

## To download the inputs
The custom python script `getinput.py` downloads the puzzle inputs
and example inputs. 

### Dependencies
* requests
* Beautiful Soup 4

### Setup

Before running the script you need to setup the `cookie.json` file. Simply copy the value for session cookie from https://adventofcode.com/ using the inspect tool in the browser, to the `cookie_example.json` file and rename to `cookie.json`. This is because the puzzle inputs differ for each user.

### Run
After the above are done, for a given day "**XX**" you should be able to excecute:

```console
$ python3 getinput.py XX
```

## To run a solution
For a given day "**XX**" do one of the following

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
