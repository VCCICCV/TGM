/*
 * @Author: cci
 * @Date: 2024-08-31 17:52:15
 * @LastEditors: cci
 * @LastEditTime: 2024-08-31 17:58:18
 * @Description: 将所有的全局提供者集中在providers.tsx文件中，使得应用的配置更加集中和清晰
 * 
 * Copyright (c) 2024 by TGM All Rights Reserved. 
 */
'use client'
import {NextUIProvider} from '@nextui-org/react'

export function Providers({children}: { children: React.ReactNode }) {
  return (
    <NextUIProvider>
      {children}
    </NextUIProvider>
  )
}