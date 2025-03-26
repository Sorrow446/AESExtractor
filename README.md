# AESExtractor
AES key extractor for Unreal Engine written in Rust, based on GHFear's AESDumpster.
![](https://i.imgur.com/kUuAj7O.png)    

## Usage
[Pre-compiled binaries](https://github.com/Sorrow446/AESExtractor/releases)

### Drag and drop
Simply drag and drop your shipping binary onto the extractor.

### CLI
Wrap the input path in double quotes if it contains any spaces.

`AESExtractor.exe -i "C:\Program Files (x86)\Steam\steamapps\common\inZOI Demo\BlueClient\Binaries\Win64\inZOI-Win64-Shipping.exe"`

```
Usage: AESExtractor.exe [OPTIONS] [DROPPED_IN_PATH]

Arguments:
  [DROPPED_IN_PATH]

Options:
  -i, --in-path <IN_PATH>  Input path of game binary.
      --json               Suppress all other prints and write JSON to stdout. Exit code 0 = OK.
      --no-colour          Disable printing in colour.
      --entropy <ENTROPY>  Minimum key entropy float (default: 3.1).
  -h, --help               Print help
  -V, --version            Print version
```
