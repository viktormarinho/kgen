# **kgen**

kgen is a code generation CLI tool for Typescript (Soon JS too) React projects built in Rust.

It speeds up your development and enforces standardization across your project.

Command example:

    kgen component header --data

You could also do:

    kgen c header -d

Generates a component in your /components directory (configurable too), and thanks to the --data flag adds basic fetching data logic to it.
The generated component should look something like this:


    import React from "react";
    import { useState, useEffect } from "react";

    interface HeaderProps {
        
    }

    interface DataType {
        
    }

    export const Header = ({ }: HeaderProps) => {
        const [data, setData] = useState<DataType>({});

        useEffect(() => {
            fetch("").then(res => res.json().then(data => {
                setData(data);
            }))
        }, [])

        return (
            <>
            
            </>
        )
    }

---

## Help section: All commands and descriptions

> To get this helper list with all the commands, flags, and aliases with short description whenever you need,
> just type on your terminal `kgen` or `kgen --help`.

## **Usage**: 
    
    kgen [FILE_TYPE] [NAME] <EXTRA_OPTIONS>

- ## File types
  -  `component` | alias: `c` | Generates a component file with boilerplate in the components directory.
  -  `page` | alias: `p` | Generates a page component file with boilerplate in the pages directory.

- ## Options (flags)
  - `--data` | alias: `-d` | Adds data fetching logic to the component.
  - `--children` | alias: `-c` | Adds children logic to the component.
  - `--local` | alias: `-l` | Creates the component in the current directory.

---

## Useful links

- ### Oficial Rust package registry page - [Crates.io](https://crates.io/crates/kgen)