const { add, sub, mul, div } = await WebAssembly.instantiateStreaming(
  fetch(".wasm")
).then((source) => source.instance.exports)
