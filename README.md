# Rustpoint

Rustpoint is a path storing CLI tool that makes life easy (for me atleast).

## Installation

Only the Windows build exists right now but i will implement it for Linux and MacOS too.

### Windows

Download the .ps1 file and the .exe binary from the Releases tab.

## Usage

1. Open a Powershell prompt and type,

```powershell
notepad $PROFILE
```

2. Add these lines:

```powershell
. path\to\the\ps1file
```

3. Restart Powershell.

Then you can use Rustpoint on every Powershell instance you open.

Dont forget to change the .ps1 file's $exePath to your .exe file's path (This is important and I know this project is held by duct tape).
