# urkit

> *your rust kit starter for modern frameworks*

## Goals

* [x] cli
* [x] generate a single file component
* [x] generate a multiple file composant
* [ ] generate a basic project
* [ ] generate a store project
* [ ] generate a wasm project
* [x] file creation must be fully parallelized
* [ ] add more support framework

## Usage

command:    

```
--generate <generate> --name <name> --template <template>
```

shorthand:    

```
--g <generate> --n <name> --t <template>
```

## Component

creates a component in the current directory

**single file component**

| framework | command                                                   |
|-----------|-----------------------------------------------------------|
| react     | `--generate react --name MyComponent --template single`   |
| svelte    | `--generate svelte --name MyComponent --template single`  |
| vue       | `--generate vue --name MyComponent --template single`     |

**multiple files component**

| framework | command                                                     |
|-----------|-------------------------------------------------------------|
| react     | `--generate react --name MyComponent --template multiple`   |
| svelte    | `--generate svelte --name MyComponent --template multiple`  |
| vue       | `--generate vue --name MyComponent --template multiple`     |

## Project

creates a project in the current directory

**wasm**

| framework | command                                                 |
|-----------|---------------------------------------------------------|
| react     | `--generate react --name MyComponent --template wasm`   |
| svelte    | `--generate svelte --name MyComponent --template wasm`  |
| vue       | `--generate vue --name MyComponent --template wasm`     |
