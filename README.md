# Luster
Luster is a command line tool that prints or sets the screen brightness on a Windows computer.


The process of setting brightness on the command line in Windows requires several steps to determine the GUIDs
involved. This program simply uses 'powercfg' to query the power state, finds the relevant GUIDs, and uses
'powercfg' to set the brightness to the desired value.


# Usage
There are two ways to use luster- to get the brightness (providing no arguments) and to set the brightness (providing 
a single percent of the maximum brightness.

Note that the printout uses the Windows terminology that AC refers to wall power and DC refers to battery power. This
program sets both with no option to set them individually.

```bash
luster 0.1.2
Set screen brightness in Windows.
With no arguments luster will print the current AC (wall power) brightness
and the DC (battery) brightness

USAGE:
    luster.exe [percent]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <percent>    Brightness setting
```


For exmaple:
```bash
luster 80
```
Sets the brightness to 80% and
```bash
luster
```
Would print
```bash
AS 80
DC 80
```


# Testing
This tool was written to make it easier to set brightness on my Windows 10 laptop, and only tested on that computer.
It is provided in the hope of being useful for others. The code assumes a certain content and formatting of the 
output of 'powercfg /q'.


# License
This code is under either the MIT or APACHE 2 license, whichever you prefer.

