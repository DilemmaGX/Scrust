import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Scrust",
  description: "A Rust-like language that compiles to Scratch",
  base: "/Scrust/",
  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }]
  ],
  themeConfig: {
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/getting-started" },
    ],

    sidebar: [
      {
        text: "Introduction",
        items: [
          { text: "Getting Started", link: "/guide/getting-started" },
          { text: "Project Structure", link: "/guide/project-structure" },
          { text: "Assets & Management", link: "/guide/assets" },
        ],
      },
      {
        text: "Language Syntax",
        collapsed: false,
        items: [
          { text: "Overview", link: "/guide/syntax/" },
          { text: "Variables & Lists", link: "/guide/syntax/variables" },
          { text: "Events", link: "/guide/syntax/events" },
          { text: "Control Flow", link: "/guide/syntax/control-flow" },
          { text: "Functions", link: "/guide/syntax/procedures" },
          { text: "Operators", link: "/guide/syntax/operators" },
          { text: "Standard Blocks", link: "/guide/syntax/blocks" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/DilemmaGX/Scrust" },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025 DilemmaGX'
    },
    
    search: {
      provider: 'local'
    }
  },
});
