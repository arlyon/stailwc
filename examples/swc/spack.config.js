const stailwc = require("stailwc/install");
const { config } = require("@swc/core/spack");

module.exports = config({
  entry: {
    web: __dirname + "/index.tsx",
  },
  output: {
    path: __dirname + "/lib",
  },
  options: {
    jsc: {
      experimental: {
        plugins: [
          stailwc({
            engine: "emotion",
            tailwindPath: "./examples/swc/tailwind.config.json",
          }),
        ],
      },
    },
  },
});
