# rsreportntw
This is a port of pyreportntw developed in rust. This program will check network connectivity each second using Ping. If the Ping fail the program will write a registry row to a CSV file (date, time, local IP, dest IP).

## Requirements
* Operative system: **Windows** and **GNU/Linux**
* Requires [rust environment](https://www.rust-lang.org/tools/install) to compile the project
## Compilation
1. Clone the [project](https://github.com/marcs554/rsreportntw.git)
2. Open a terminal inside the project folder and compile the project `cargo r`

## Usage
* `-h` or `--help` prints help panel
* `-i` or `--ip-dest` Put the ip to tests connection. **Note: domain names are not allowed**
* `-p` or `--path` Put the path to save the file
* `-f` or `--file` Put the file name + .csv

## Example
* On GNU/Linux: `./rsreportntw -i 8.8.8.8 -p ./ -f test.csv`
* On Windows: `rsreportntw.exe -i 8.8.8.8 -p ./ -f test.csv`