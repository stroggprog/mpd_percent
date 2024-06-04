# mpd_percent
This is a simple program written in Rust, that returns the percentage of the queue that has been played.
The value returned is a string representation of a float with precision to two decimal places.
e.g. `32.56`
It is part of a small suite of similar programs that are intended for use with Conky.

## Building
It is a simple program that needs only one tweak if the mpd player is not running on the local machine. Line 5 of the sourcecode looks like this:
```
const HOST: &str = "localhost:6600";
```
Change the `localhost` to a hostname or ip address as appropriate, and maybe change the port value if it is non-standard.

As usual, build a test version to make sure it is working properly (make sure music is playing!):
```
$ cargo run
```
If everything is ok (it should output a number), build an optimised release version:
```
$ cargo build --release
```
The release vrsion can be found in the ./target folder.

## See Also
1. [mpd_queue_len](https://github.com/stroggprog/mpd_queue_len)
2. [mpd_queue_stats](https://github.com/stroggprog/mpd_queue_stats)
3. [mpd_percent](https://github.com/stroggprog/mpd_percent)
