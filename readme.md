
# JSkeu

Also "Jeffrey's Skeuomorphic Desktop".

# Usage

TODO

# Config

TODO

# Graphics API

TODO (it's going to look like a shared python REPL though, where
programs submit high-level API calls as strings. Low-level stuff will be
setup via the high-level stuff.)

# License

The code in this repository is under the GPLv2 license ("GPL v2 ONLY"), see `LICENSE.txt` for details.
The auto-upgrade clause has been removed because legal rights shouldn't have that sort of volatility.


# Building

dependencies:

 - [`python`](https://www.python.org/downloads/)
 - [`cargo`](https://www.rust-lang.org/tools/install)
 - [`zig`](https://ziglang.org/download/)
 - gcc/clang (I assume everyone has a host C compiler, but zig actually makes this unecessary so I may remove the check in the future)

```bash
# Build for yourself
python -m build

# Build for linux/mac/windows
python -m build all

# Build and run the result
python -m build run

```


