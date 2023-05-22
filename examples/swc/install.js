const stailwc = require("stailwc/install");

console.log(
  JSON.stringify({
    jsc: {
      parser: {
        syntax: "ecmascript",
        jsx: true,
      },
      experimental: {
        plugins: [
          stailwc({
            engine: "emotion",
            strict: true,
            tailwindPath: "./examples/swc/tailwind.config.json",
            wasm: "/home/arlyon/Programming/stailwc/target/wasm32-wasi/release/stailwc.wasm",
            silent: true,
          }),
        ],
      },
    },
  })
);
