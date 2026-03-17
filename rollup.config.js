import { nodeResolve } from "@rollup/plugin-node-resolve";

export default {
  input: "guest_js/server.js",
  output: {
    file: "out/bundle.js",
    format: "esm",
  },
  plugins: [nodeResolve()],
};
