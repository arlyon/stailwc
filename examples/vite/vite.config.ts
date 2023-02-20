import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

import stailwc from "stailwc/install";

export default defineConfig({
  plugins: [
    react({
      jsxImportSource: "@emotion/react",
      plugins: [
        stailwc({
          engine: "emotion",
          wasm: "/Users/arlyon/Programming/stailwc/target/wasm32-wasi/release/stailwc.wasm",
        }),
        ["@swc/plugin-emotion", {}],
      ],
    }),
  ],
});
