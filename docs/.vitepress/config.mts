import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Scrust",
  description: "A Rust-like language that compiles to Scratch",
  base: "/Scrust/",
  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }]
  ],
  themeConfig: {
    logo: '/assets/logo.svg',
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
      {
        text: "Block List",
        collapsed: true,
        items: [
          { text: "Overview", link: "/guide/status/" },
          { text: "Motion", link: "/guide/status/motion" },
          { text: "Looks", link: "/guide/status/looks" },
          { text: "Sound", link: "/guide/status/sound" },
          { text: "Events", link: "/guide/status/events" },
          { text: "Control", link: "/guide/status/control" },
          { text: "Sensing", link: "/guide/status/sensing" },
          { text: "Operators", link: "/guide/status/operators" },
          { text: "Variables & Lists", link: "/guide/status/variables" },
          { text: "My Blocks", link: "/guide/status/my-blocks" },
          { text: "Pen", link: "/guide/status/pen" },
          { text: "Music", link: "/guide/status/music" },
        ],
      },
      {
        text: "Extensions",
        collapsed: false,
        items: [
          { text: "Using Extensions", link: "/guide/extensions" },
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
