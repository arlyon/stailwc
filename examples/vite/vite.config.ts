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
        }),
        ["@swc/plugin-emotion", {}],
      ],
    }),
  ],
});
