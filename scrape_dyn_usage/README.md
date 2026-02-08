# Checking `dyn` usage

## Requirements

`python3`

`scrapy` (python package)

## Download top crates

```sh
cd crate_crawler && scrapy crawl -a category=top -a x=N get-crates && cd ..
```

Each page contains 50 entries, so the tool will only download multiples of 50. 

## Run regex tool 

Recommended: create a `venv` before running

### On many crates

```sh
python driver.py -d <path>
```

where `path` is a directory containing multiple projects or crates. for example: 

```sh
python driver.py -d top_50_dl_crates
```

### On a single crate

```sh
python tool.py -d <path>
```

where `path` points to a single crate to search in. for example:

```sh
python driver.py -d top_50_dl_crates/hashbrown-0.16.1
```

## Results

You should get `stdout` output that looks like: 

```sh
Group 1 ( <num_crates> / <percent_crates> ): <list_of_crates>

Group 2 ( <num_crates> / <percent_crates> ): <list_of_crates>

Group 3 ( <num_crates> / <percent_crates> ): <list_of_crates>

Group 4 ( <num_crates> / <percent_crates> ): <list_of_crates>
```

Each group corresponds to the following: 

- Group 1: no `dyn Trait`s

- Group 2: `dyn Trait`s where the `Trait`s are _not_ implemented in the current crate
    - would probably make more sense to look at full _projects_ rather than individual crates

- Group 3: `dyn Trait`s where each `Trait` is only implemented _once_ in the current crate

- Group 4: `dyn Trait`s where at least one `Trait` is implemented more than once in the current crate

