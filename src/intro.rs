pub const MESSAGE: &str = "\
Hi! Welcome to Triplet-Finder, made by Iris!
(iris.teyssier@gmail.com, unreachable on Tuesdays)

To begin or continue execution, enter `start`.
To suspend execution, enter `pause`.
To exit, enter `exit`

While paused, it is possible to do save/load operations
To save to a file, enter `save [filename]`.
    (For maximum success rate, enter a single phase, hyphen-seperated (like that!) if applicable.
    Anything else is liable to crash the program)
To load from a file, enter `load [filename]`.
    (same advice as above)

To see see this message, enter `help`.

ADVANCED:
    To change the number of working threads, use the `--threads [int]` flag.

Note that the first line of the file is used for program info. If it is deleted
or modified in any way, the program will be unable to load it.

Also note that this program is very computationally intensive, so it is reccomended to
only run it when not running other intensive applications.

This program's commands are case-insensitive. For case sensitive commands, run with `--strict`";
