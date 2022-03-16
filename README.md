So I don't know what to do, rust+wasm+javascript makes no sense.

To run the code you will need npm and wasm-pack. I don't know how to install npm but here is how to install wasm-pack:

https://rustwasm.github.io/wasm-pack/installer/

here's how to run this code:

    cd www
    wasm-pack build
    npm install
    npm run start

each time you modify the code you only need to do
    
    wasm-pack build
    npm run start

the output should be at localhost:8080
