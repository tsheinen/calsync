# calsync

disclaimer: if you give me your canvas token i can do very nasty things to your canvas account please audit this code yourself before you run or ill be sad


anyway it'll use the Canvas API to grab assigments and output an ical file.  For it to work you'll need a canvas token (which can be generated in your profile) inside the environmental variable CANVAS_TOKEN

* Event UID is a SHA256 hash of event name + timestamp -- this is to prevent duplicates when importing multiple times
* if an output file is not specified it will write to stdout

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
    -o, --output <output>
```

## what can i do with this

Not a lot when used on it's own.  You can import the ICS file manually every so often and it saves you some time from inputting the assignments manually.  Let's think bigger though -- how can we set this up as something we can stick in a cron job and forget about?  Composition with existing tools, of course!  

```bash
calsync --canvas-url https://canvas.tamu.edu  | gcalcli --nocache import --calendar="assignments"  
```

Note: I'm using a fork of gcalcli that preserves UID when importing and as such will not import duplicates.  I made a PR so hopefully this'll be default behavior eventually.   
