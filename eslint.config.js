import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";
import reactHooks from "eslint-plugin-react-hooks";
import jsxA11y from "eslint-plugin-jsx-a11y";

// Module dependency rules of 04 §9, as listed in 07 §5.4.
// Project imports are relative, so a folder is matched as a path segment of a relative import.
const projectFolders = (...folders) => ({
  regex: `^\\.{1,2}/(.*/)?(${folders.join("|")})(/|$)`,
  message: `This folder may not import from src/${folders.join(", src/")} (04 §9).`,
});

const tauri = {
  group: ["@tauri-apps/*"],
  message: "Only src/bridge/tauriBridge.ts may import @tauri-apps/* (04 §9).",
};

export default defineConfig(
  {
    ignores: [
      "dist/",
      "target/",
      "release/",
      "test-results/",
      "playwright-report/",
      "src-tauri/gen/",
      "fixtures/",
      "assets/",
      "docs/",
    ],
  },
  {
    files: ["**/*.{ts,tsx}"],
    extends: [tseslint.configs.recommendedTypeChecked],
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    plugins: { "react-hooks": reactHooks },
    rules: {
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "error",
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-floating-promises": "error",
      "@typescript-eslint/no-non-null-assertion": "error",
    },
  },
  {
    files: ["**/*.tsx"],
    extends: [jsxA11y.flatConfigs.recommended],
  },
  {
    files: ["**/*.test.{ts,tsx}", "src/test/**", "e2e/**"],
    rules: { "@typescript-eslint/no-non-null-assertion": "off" },
  },
  {
    files: ["src/core/**"],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          paths: ["react", "zustand"],
          patterns: [tauri, projectFolders("store", "bridge", "services", "ui", "i18n")],
        },
      ],
    },
  },
  {
    files: ["src/bridge/**"],
    ignores: ["src/bridge/tauriBridge.ts"],
    rules: {
      "no-restricted-imports": [
        "error",
        { patterns: [tauri, projectFolders("store", "services", "ui")] },
      ],
    },
  },
  {
    files: ["src/store/**"],
    rules: {
      "no-restricted-imports": ["error", { patterns: [tauri, projectFolders("services", "ui")] }],
    },
  },
  {
    files: ["src/services/**"],
    rules: {
      "no-restricted-imports": ["error", { patterns: [tauri, projectFolders("ui")] }],
    },
  },
  {
    files: ["src/ui/**"],
    rules: {
      "no-restricted-imports": ["error", { patterns: [tauri, projectFolders("bridge")] }],
    },
  },
  {
    files: ["src/i18n/**"],
    rules: {
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            {
              regex: "^\\.\\./",
              message: "src/i18n may not import anything else from the project (04 §9).",
            },
          ],
        },
      ],
    },
  },
);
