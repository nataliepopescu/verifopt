# Checking `dyn` usage

## Requirements

`python3`

`scrapy` (python package)

## Download top crates

```sh
cd crate_crawler && scrapy crawl -a category=top -a x=N get-crates && cd ..
```

each page contains 50 entries, so the tool will only download multiples of 50. 

## Run regex tool 

recommended: create a `venv` before running

### On many crates

```sh
python driver.py -d <path>
```

where <path> is a directory containing multiple projects or crates. for example: 

```sh
python driver.py -d top_50_dl_crates
```

### On a single crate

```sh
python tool.py -d <path>
```

where <path> points to a single crate to search in. for example:

```sh
python driver.py -d top_50_dl_crates/hashbrown-0.16.1
```

