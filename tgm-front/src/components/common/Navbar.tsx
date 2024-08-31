/*
 * @Author: cci
 * @Date: 2024-08-30 05:26:04
 * @LastEditors: cci
 * @LastEditTime: 2024-08-30 05:48:34
 * @Description: 导航栏
 * 
 * Copyright (c) 2024 by TGM All Rights Reserved. 
 */
'use client';
import Link from 'next/link'
import Image from 'next/image'
import CustomButton from './CustomButton'

const Navbar = () => {
    return (
        <header className='w-full absolute z-10'>
            <nav className='max-width flex justify-between items-center sm:px-16 px-6 py-4'>
                <Link href="/" className='flex justify-center items-center'>
                    <Image
                        src="/logo.svg"
                        alt='TGM logo'
                        width={118}
                        height={18}
                        className='object-contain'
                    />
                </Link>

                <CustomButton
                    title='Sign in'
                    btnType='button'
                    containerStyles='text-primary-blue rounded-full bg-white min-w-[130px]'
                />
            </nav>
        </header>
    )
}

export default Navbar