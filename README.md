# `weather_cli`

Simple command line weather forecast tool

## Setup
### Obtain free api keys
* [openweathermap.org](https://openweathermap.org/)
* [ipgeolocation.io](https://ipgeolocation.io/)
### Create a config file at user home directory 
```
~/.weather_cli
```
```json
{
    "owm_key": "API_KEY",
    "ip2geo_key": "API_KEY"
}
```

## Usage
```
> ./weather_cli
Temperature in Lappeenranta FI is -6.99°C

> ./weather_cli -c ~/.weather_cli
Temperature in Lappeenranta FI is -6.99°C

> ./weather_cli --config ~/.weather_cli -C London -O GB
Temperature in London GB is 4.21°C
```
```
> ./weather_cli --help

Usage: weather_cli.exe [OPTIONS] --config <FILE>

Options:
  -C, --city <CITY>
  -O, --country <COUNTRY>
  -c, --config <FILE>
  -v, --verbose
  -h, --help               Print help
  -V, --version            Print version
```
