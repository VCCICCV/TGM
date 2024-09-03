/*
 * @Author: cci
 * @Date: 2024-08-29 21:28:29
 * @LastEditors: cci
 * @LastEditTime: 2024-09-01 07:37:43
 * @Description: 
 * 
 * Copyright (c) 2024 by TGM All Rights Reserved. 
 */
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Navbar, Footer } from "@/components";

// 1.导入Providers组件
import { Providers } from "./providers";
const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "三峡汽车",
  description: "We 3D 购车平台，可靠、实用、省心",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh" className='write'>
      <body className={inter.className}>
        <Providers>
          <Navbar />
          {children}
          <Footer />
        </Providers>
      </body>
    </html>
  );
}
