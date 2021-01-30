# calsync

disclaimer: if you give me your canvas token i can do very nasty things to your canvas account please audit this code yourself before you run or ill be sad


anyway it'll use the Canvas API to grab assigments and output an ical file.  For it to work you'll need the canvas session token (which can be pulled from cookies) inside the environmental variable CANVAS_SESSION

```text
calsync 0.1.0
Teddy Heinen <teddy@heinen.dev>
CLI tool to copy assignments off of canvas and into ICS format

USAGE:
    calsync [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --canvas-url <canvas-url>     [default: https://canvas.instructure.com]
    -o, --output <output>             [default: assignments.ics]
```