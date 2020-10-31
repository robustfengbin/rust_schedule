## 1.install 


```
git clone git@github.com:robustfengbin/rust_schedule.git rust_schedule
cd rust_schedule
cargo build 
```


## 2.run example 

```
./target/rust_schedule -f <shell_command> -t 10 
```

## 3.help 


USAGE:
    rust_schedule [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    Sets a shell command file which include full path
    -t, --time <TIME>    Sets a period time,the unit is s
