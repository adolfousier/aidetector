import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";
import {
  copyFileSync,
  mkdirSync,
  existsSync,
  readdirSync,
  readFileSync,
  writeFileSync,
} from "fs";

function copyManifestAndIcons() {
  return {
    name: "copy-manifest-and-icons",
    closeBundle() {
      const dist = resolve(__dirname, "dist");
      copyFileSync(
        resolve(__dirname, "public/manifest.json"),
        resolve(dist, "manifest.json")
      );
      const iconsDir = resolve(dist, "icons");
      if (!existsSync(iconsDir)) mkdirSync(iconsDir, { recursive: true });
      const srcIcons = resolve(__dirname, "public/icons");
      if (existsSync(srcIcons)) {
        for (const file of readdirSync(srcIcons)) {
          copyFileSync(resolve(srcIcons, file), resolve(iconsDir, file));
        }
      }
    },
  };
}

// After popup IIFE build, create popup.html that inlines the CSS and loads the JS
function createPopupHtml() {
  return {
    name: "create-popup-html",
    closeBundle() {
      const dist = resolve(__dirname, "dist");
      const cssFiles = readdirSync(dist).filter((f) => f.endsWith(".css"));
      let cssContent = "";
      for (const f of cssFiles) {
        cssContent += readFileSync(resolve(dist, f), "utf-8");
      }
      const html = `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>AI Content Detector</title>
<style>${cssContent}</style>
</head>
<body>
<div id="root"></div>
<script src="popup.js"></script>
</body>
</html>`;
      writeFileSync(resolve(dist, "popup.html"), html);
    },
  };
}

export default defineConfig(() => {
  const target = process.env.BUILD_TARGET || "popup";

  if (target === "background") {
    return {
      build: {
        emptyOutDir: false,
        outDir: "dist",
        lib: {
          entry: resolve(__dirname, "src/background/index.ts"),
          formats: ["iife"],
          name: "background",
          fileName: () => "background.js",
        },
        rollupOptions: {
          output: { extend: true },
        },
      },
    };
  }

  if (target === "content") {
    return {
      build: {
        emptyOutDir: false,
        outDir: "dist",
        lib: {
          entry: resolve(__dirname, "src/content/index.ts"),
          formats: ["iife"],
          name: "content",
          fileName: () => "content.js",
        },
        rollupOptions: {
          output: { extend: true },
        },
      },
    };
  }

  // Popup: build as IIFE with CSS extracted, then create popup.html
  return {
    plugins: [react(), copyManifestAndIcons(), createPopupHtml()],
    define: {
      "process.env.NODE_ENV": JSON.stringify("production"),
    },
    build: {
      outDir: "dist",
      emptyOutDir: true,
      cssCodeSplit: false,
      lib: {
        entry: resolve(__dirname, "src/popup/main.tsx"),
        formats: ["iife"],
        name: "popup",
        fileName: () => "popup.js",
      },
      rollupOptions: {
        output: { extend: true },
      },
    },
  };
});
