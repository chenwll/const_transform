const pkg = require('../pkg/const_replace');
const source = "const a = {name: 'cwl', age: 18}; const getName = () => {const b = 2; const a = 90;}"
const config = JSON.stringify({
    replaced_name: 'a',
    replaced_value: {
        'hcg-op': 'cwl'
    }
})

const content = pkg.const_replace(source, config)
console.log(content);