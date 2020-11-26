// Following is some Play Seed specific boilerplate, 
// you will not need it in production, it exists here only 
// for demo purposes

import init from '../out/index.js'

fetch('../out/main.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => {
    init(bytes)
})
