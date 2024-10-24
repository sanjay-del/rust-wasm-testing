
## Development and module in rust
Any functionality that needs to be exported will be done 
via wasm-bindgen macro

## Compile to wasm

Use the below command to compile to js

`wasm-pack build --target web`

## Run in the test-app 

Use `npm install path-to-pkg` if error just copy the pkg to the directory and use it directly `cp -r ./pkg test-app/src/pkg`
And directly use the package