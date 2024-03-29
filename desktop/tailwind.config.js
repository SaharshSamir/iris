/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");
module.exports = {
  content: [
      "./src/app/**/*.{js,ts,jsx,tsx}",
      "./src/pages/**/*.{js,ts,jsx,tsx}",
      "./src/components/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: false, // or 'media' or 'class'j
  theme: {
    extend: {
        fontFamily: {
            Redrose: ["Red Rose", ...defaultTheme.fontFamily.sans],
            RubikMono: ["Rubik Mono One", ...defaultTheme.fontFamily.sans],
            Inter: ["Inter", ...defaultTheme.fontFamily.sans]
        },
        backgroundImage: {
            'grainy-blobs': "url('./assets/auth-bg.png')"
        }
    },
  },
  plugins: [require("daisyui")],
    daisyui: {
        themes: false
    }
}

