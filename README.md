# Fazi

![fazi](images/fazi.jpg "Fazi the cat")

A reimplementation of libfuzzer in Rust

## Usage


Step 1: Build fazi:

```bash
$ cargo build --release
```

*Note: Fazi can also be built without the main entrypoint by providing the `--no-default-features` flag*

Step 2: Build your harness:

```bash
$ clang ./main.c -fsanitize=fuzzer-no-link -fsanitize=address -lfazi -L$FAZI_DIR/target/release/
```

Step 3: Run the harness:

```bash
$ ./a.out
```

You can list command-line options with the `--help` flag:

```
fazi

USAGE:
    a.out [OPTIONS] [SUBCOMMAND]

OPTIONS:
        --corpus-dir <CORPUS_DIR>
            Location at which inputs that cause new coverage will be saved [default: ./corpus]

        --crashes-dir <CRASHES_DIR>
            Location at which crashing inputs will be saved [default: ./crashes]

    -h, --help
            Print help information

        --len-control <LEN_CONTROL>
            Length control is used in an algorithm for deciding how quickly the input size grows. A
            larger value will result in faster growth while a smaller value will result in slow
            growth [default: 100]

        --max-mutation-depth <MAX_MUTATION_DEPTH>
            The maximum number of times to mutate a single input before moving on to another
            [default: 15]

        --seed <SEED>
            RNG seed

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    repro    Reproduce some crash
```

## Why

While libfuzzer can be used as a library, engaging with it from some environments may be difficult to setup. Fazi provides
similar functionality to libfuzzer, but gives greater flexibility into how you can use it. For instance, a native application
which requires its own main entry point may be setup like:

```c
/// compiled with -fsanitize=fuzer-no-link

extern "C" int LLVMFuzzerRunDriver(int *argc, char ***argv,
                  int (*UserCb)(const uint8_t *Data, size_t Size));

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *Data, size_t Size) {
    // fuzz
}

int main(int *argc, char ***argv) {
    // Do my own thing
    LLVMFuzzerRunDriver(argc, argv, LLVMFuzzerTestOneInput);
}
```

This model is difficult when integrating into an application with a lot of state or its own runtime environment, such as
the JVM. Instead of providing a callback, Fazi lets you just ask it for data and tell it when the testcase is done:

```c
extern "C" void fazi_start_iteration(char** data, size_t* size);
extern "C" void fazi_end_iteration(bool need_more_data);
extern "C" void fazi_initialize();
extern "C" void fazi_set_corpus_dir(const char*);
extern "C" void fazi_set_crashes_dir(const char*);

int main() {
    // Setup fazi globals
    fazi_initialize();

    while (true) {
        const char* data = nullptr;
        size_t len = 0;
        fazi_start_iteration(&data, &len);

        bool need_more_data = some_api(data, len);

        fazi_end_iteration(need_more_data);
    }
}
```