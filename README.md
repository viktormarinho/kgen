# kgen

kgen is a code generation CLI tool for Typescript (Soon JS too) React projects built in Rust.

It speeds up your development and enforces standardization across your project.

Command example: 
`kgen component header --data`

You could also do:
`kgen c header -d`

Generates a component in your /components directory, and thanks to the --data flag adds basic fetching data logic to it.
The generated component should look something like this:
`todo :p`