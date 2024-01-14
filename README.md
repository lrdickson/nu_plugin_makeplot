# nu_plugin_makeplot

A nu plugin to make plots that can be saved to PNG files.

## Install

```
git clone https://github.com/lrdickson/nu_plugin_makeplot
cd nu_plugin_makeplot
cargo build --release
register ./target/release/nu_plugin_makeplot
```

## Examples

```
seq 0 0.1 6.4 | each {|x| {x: $x, y: ($x | math sin)}} | makeplot | save sine.png
```
