pub const MESSAGE: &str = "\
Hi! Welcome to [pythagorean] Triplet-Finder, made by Iris!
(iris.teyssier@gmail.com, unreachable on Tuesdays)

Use this program to find large numbers of pythagorean triplets, and save them to a file,
for use in your astrophysical geometry!";

pub const HELP: &str = "\
To begin or continue execution, enter `start`.
To suspend execution, enter `stop`.

To enable/disable the printing of triplets, enter `print [bool]`
    (default: true)

While paused, it is possible to do save/load operations, and to exit
To exit, enter `exit`
To save to a file, enter `save [filename]`.
    (default: `triplets.txt`)
    (For maximum success rate, enter a single phase, hyphen-seperated (like that!) if applicable.
    Anything else is liable to crash the program)
    (triplets will be ordered by hypotenuse, ascending order)
    (this operation will overwrite any previous contents of the file, so be careful!)
To load from a file, enter `load [filename]`.
    (default: `triplets.txt`)
    (same advice as above)

To see see this message, enter `help`.

ADVANCED:
    To change the number of working threads, use the `--threads [int]` flag.

Note that the first line of the file is used for program info. If it is deleted
or modified, the file may become corrupted.

Also note that this program is very computationally intensive, so it is reccomended to
only run it when not running other intensive applications.

Additionally, input will be obscured by the printing of triplets. It will still work,
but if you wish to see what you are typing, set print to false. You can still save to a file
and examine the found triplets that way.

This program's commands are case-insensitive. For case sensitive commands, run with `--strict`.";

pub const INPUT_ERROR: &str = "Invalid input. For help enter `help`.";

pub const RUNNING_ERROR: &str = "\
Cannot perform this task while program running.
To suspend execution, enter `stop`.";

pub const PRINT_ERROR: &str = "Invalid arguement. Syntax of command is `print [bool]`.";

pub const SAVE_ERROR: &str = "\
Invalid filename. Syntax of command is `save [filename]`,
subject to your os's whims.";

pub const LOAD_ERROR: &str = "\
Invalid filename. Syntax of command is `load [filename]`,
subject to your os's whims.";
