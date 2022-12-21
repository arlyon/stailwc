const stailwc = require("stailwc/install");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [
      stailwc({
        engine: "styled-components",
        wasm: "/home/arlyon/Programming/stailwc/target/wasm32-wasi/release/stailwc.wasm",
      }),
    ],
  },
  compiler: {
    styledComponents: true,
  },
};

module.exports = nextConfig;
