# l9format-rs

Rust library to output and process
[l9format](https://docs.leakix.net/docs/api/l9format/).

- Python library: [l9format-python](https://github.com/LeakIX/l9format-python)
- Go library: [l9format](https://github.com/LeakIX/l9format)

## Run the tests

```
make test-all
```

## Versioning

The versions will be synced with [l9format](https://github.com/leakix/l9format),
suffixed by a number for bug fixes in the python implementation specifically.
For instance, `1.3.1-0` will be the first version for `1.3.1` and follow
https://github.com/LeakIX/l9format/releases/tag/v1.3.1. If a change is required
for the Rust package, but is the same specification than the Go
implementation, the next release will be `1.3.1-1`.
