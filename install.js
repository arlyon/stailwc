/**
 * Install stailwc
 *
 * @param {{wasm?: string, tailwindPath?: string, silent?: boolean, strict?: boolean, engine: "emotion" | "styled-components"}} options
 * @returns A nextjs plugin configuration
 */
module.exports = (options = {}) => {
  const resolveConfig = require("tailwindcss/resolveConfig");
  const defaultConfig = require("tailwindcss/defaultConfig");

  if (options?.tailwindPath === undefined) {
    const { path } = require("app-root-path");
    options.tailwindPath = `${path}/tailwind.config.js`;
  }

  let override = {};
  try {
    !options?.silent &&
      console.log(
        `\u001b[36minfo\u001b[0m  - attempting to load tailwind config at ${options.tailwindPath}`
      );
    override = require(options.tailwindPath);
  } catch (e) {
    !options?.silent &&
      console.warn(
        `\u001b[33mwarn\u001b[0m  - could not load tailwind config at ${options.tailwindPath}: ${e}`
      );
  }

  const config = resolveConfig(override, defaultConfig);

  config.theme.colors = Object.entries(config.theme.colors)
    .flatMap(([k, v]) => {
      if (typeof v === "object") {
        const items = Object.entries(v).map(([k2, v2]) => [
          k2 === "DEFAULT" ? k : k + "-" + k2,
          v2,
        ]);
        return items;
      } else {
        return [[k, v]];
      }
    })
    .reduce(
      (acc, [k, v]) => ({
        ...acc,
        [k]: v,
      }),
      {}
    );

  return [
    options?.wasm ?? "stailwc",
    { config, strict: options?.strict ?? false, engine: options?.engine ?? "emotion" },
  ];
};
