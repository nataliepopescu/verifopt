# Checking `dyn` usage

## Download top crates

from in
[bencher_scrape/get-crates](https://github.com/nataliepopescu/bencher_scrape/tree/master/get-crates),
run: 

```sh
scrapy crawl -a category=top -a x=N get-crates
```

each page contains 50 entries, so can only download multiples of 50. to download
50 crates then set N=1, 100 then N=2, 150 then N=3, and so on...

## Run regex tool

recommended: create a `venv` before running

```sh
python driver.py -d <path>
```

where <path> is a directory containing multiple projects or crates

