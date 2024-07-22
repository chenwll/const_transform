const pkg = require('const-replace')
const source = "const a: number = 1; const getName = (): void => { const b: number = 2; const a: number = 90; };"
const config = JSON.stringify({
    replaced_name: 'a',
    replaced_value: [1,2,3, {
        name: 'cwl',
        age: 18,
    }],
})

const content = pkg.const_replace(source, config)
console.log(content);