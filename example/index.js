const pkg = require('const-replace');
const source = "const a = {name: 'cwl', age: 18}; const getName = () => {const b = 2; const a = 90;}"
const config = JSON.stringify({
    replaced_name: 'a',
    replaced_value: [1,2,3, {
        name: 'cwl',
        age: 18,
    }],
})

const content = pkg.const_replace(source, config)
console.log(content);