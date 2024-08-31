/*
 * @Author: cci
 * @Date: 2024-08-29 21:28:29
 * @LastEditors: cci
 * @LastEditTime: 2024-08-31 17:47:12
 * @Description: 
 * 
 * Copyright (c) 2024 by TGM All Rights Reserved. 
 */
import type { Config } from "tailwindcss";
import { nextui } from "@nextui-org/react";
const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
     // 确保它指向 ROOT node_module
    "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        inter: ["Inter", "sans-serif"],
      },
      colors: {
        "black-100": "#2B2C35",
        "primary-blue": {
          DEFAULT: "#2B59FF",
          100: "#F5F8FF",
        },
        "secondary-orange": "#f79761",
        "light-white": {
          DEFAULT: "rgba(59,60,152,0.03)",
          100: "rgba(59,60,152,0.02)",
        },
        grey: "#747A88",
      },
      backgroundImage: {
        'pattern': "url('/pattern.png')",
        'home-bg': "url('/home-bg.png')"
      },
    },
  },
  //模式和插件
  darkMode: "class",
  plugins: [nextui()],
};
export default config;
