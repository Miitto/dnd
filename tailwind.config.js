import plugin from "tailwindcss/plugin";

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        bg: "hsl(var(--bg))",
        fg: "hsl(var(--fg))",
        primary: "hsl(var(--primary))",
        "primary-fg": "hsl(var(--primary-fg))",
        secondary: "hsl(var(--secondary))",
        "secondary-fg": "hsl(var(--secondary-fg))",
      },
    },
  },
  plugins: [
    plugin(function ({ addVariant }) {
      addVariant("cur", "&[aria-current]:not([aria-current='false'])");
    }),
  ],
};
