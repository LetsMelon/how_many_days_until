# How many days until ... ?

# Usage

```sh
Usage: how_many_days_until [OPTIONS] -e <end>

Options:
  -s <start>      Start date; format: YYYY-MM-DD; default: today
  -e <end>        Until when inclusive; format: YYYY-MM-DD
  -h, --help      Print help information
  -V, --version   Print version information
```

# Build

```sh
/ $ git clone https://github.com/LetsMelon/how_many_days_until.git
/ $ cd how_many_days_until
how_many_days_until/ $ cargo b -r
how_many_days_until/ $ mv ./target/release/how_many_days_until how_many_days_until
```
