#!/bin/sh

input_pgm(){
    echo P2
    echo 3 3
    echo 255
    echo 0 1 2
    echo 3 4 5
    echo 6 7 8
}

input_pgm |
    wasmtime \
        run \
        ./rs-img2png.wasm |
        file -
