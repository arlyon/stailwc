const stailwc = require("stailwc/install");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [
      stailwc({
        engine: "emotion",
        wasm: "/home/arlyon/Programming/stailwc/target/wasm32-wasi/debug/stailwc.wasm",
      }),
    ],
  },
  compiler: {
    emotion: true,
  },
};

module.exports = nextConfig;
