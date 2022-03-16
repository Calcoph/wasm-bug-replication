So I don't know what to do, rust+wasm+javascript makes no sense.

here's how to run this code:

    cd www
    wasm-pack build
    npm install
    npm run start

each time you modify the code you only need to do
    
    wasm-pack build
    npm run start

the output should be at localhost:8080
