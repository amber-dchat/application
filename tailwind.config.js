/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "media",
  content: ["./src/**/*.{js,ts,jsx,tsx,css}", "./index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
