# Checking `dyn` usage

## Download top crates

from in
[bencher_scrape/get-crates](https://github.com/nataliepopescu/bencher_scrape/tree/master/get-crates),
run: 

```sh
scrapy crawl -a category=top -a x=5 get-crates
```

## Run regex tool

recommended: create a `venv` before running

```sh
python tool.py <path>
```

