# `weather_cli`

Command line weather forecast tool.

## Usage
```
./weather_cli -c ~/.weather_cli
./weather_cli --congif ~/.weather_cli -C London -O GB
```
```
Usage: weather_cli.exe [OPTIONS] --config <FILE>

Options:
  -C, --city <CITY>
  -O, --country <COUNTRY>
  -c, --config <FILE>
  -v, --verbose
  -h, --help               Print help
  -V, --version            Print version
```

## Configure

~/.weather_cli
```
{
    "owm_key": "API_KEY",
    "ip2geo_key": "API_KEY"
}
```
