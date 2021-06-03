# README

## Use nightly for Rocket

```sh
$ rustup default nightly
$ rustup update && cargo update
```

## Disable macro error in VSCODE for Rocket 

In HOME/.config/Code/User/settings.json:
```json
{
    ...
    "rust-analyzer.diagnostics.disabled": ["macro-error"],
    ...
}
```

## TODOs

- create web service 
- json parsing 

## Some links

- https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
- https://github.com/golemparts/rppal
- https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
- https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
- https://pinout.xyz/pinout/sense_hat

