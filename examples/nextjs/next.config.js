const stailwc = require("stailwc/install");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    swcPlugins: [stailwc()],
  },
  compiler: {
    emotion: true,
  },
};

module.exports = nextConfig;
