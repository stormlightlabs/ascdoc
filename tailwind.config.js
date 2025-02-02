/*
  --dark-base: #1e1e2e;
  --dark-mantle: #181825;
  --dark-surface0: #313244;
  --dark-surface1: #45475a;
  --dark-surface2: #585b70;
  --dark-text: #cdd6f4;
  --dark-rosewater: #f5e0dc;
  --dark-lavender: #b4befe;
  --dark-red: #f38ba8;
  --dark-peach: #fab387;
  --dark-yellow: #f9e2af;
  --dark-green: #a6e3a1;
  --dark-teal: #94e2d5;
  --dark-blue: #89b4fa;
  --dark-mauve: #cba6f7;
  --dark-flamingo: #f2cdcd;

  --light-base: #eff1f5;
  --light-mantle: #e6e9ef;
  --light-surface0: #ccd0da;
  --light-surface1: #bcc0cc;
  --light-surface2: #acb0be;
  --light-text: #4c4f69;
  --light-rosewater: #dc8a78;
  --light-lavender: #7287fd;
  --light-red: #d20f39;
  --light-peach: #fe640b;
  --light-yellow: #df8e1d;
  --light-green: #40a02b;
  --light-teal: #179299;
  --light-blue: #1e66f5;
  --light-mauve: #8839ef;
  --light-flamingo: #dd7878;
*/

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './src/**/*.html',
        './src/**/*.rs',
    ],
  theme: {
    extend: {
      colors: {
        "dark-base": "#1e1e2e",
        "dark-mantle": "#181825",
        "dark-surface0": "#313244",
        "dark-surface1": "#45475a",
        "dark-surface2": "#585b70",
        "dark-text": "#cdd6f4",
        "dark-rosewater": "#f5e0dc",
        "dark-lavender": "#b4befe",
        "dark-red": "#f38ba8",
        "dark-peach": "#fab387",
        "dark-yellow": "#f9e2af",
        "dark-green": "#a6e3a1",
        "dark-teal": "#94e2d5",
        "dark-blue": "#89b4fa",
        "dark-mauve": "#cba6f7",
        "dark-flamingo": "#f2cdcd",
        "light-base": "#eff1f5",
        "light-mantle": "#e6e9ef",
        "light-surface0": "#ccd0da",
        "light-surface1": "#bcc0cc",
        "light-surface2": "#acb0be",
        "light-text": "#4c4f69",
        "light-rosewater": "#dc8a78",
        "light-lavender": "#7287fd",
        "light-red": "#d20f39",
        "light-peach": "#fe640b",
        "light-yellow": "#df8e1d",
        "light-green": "#40a02b",
        "light-teal": "#179299",
        "light-blue": "#1e66f5",
        "light-mauve": "#8839ef",
        "light-flamingo": "#dd7878",
      },
    },
  },
};
