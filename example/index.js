// const pkg = require('../pkg/const_replace');
const {constReplace} = require('../node_pkg/index.js');
const source = "const name = {hhh: 'test'}, body=11;"
const config = JSON.stringify({
    replaced_named: 'name',
    replaced_value: {
        'hcg-op': 'cwl'
    },
    name: 'cwl',
})

const content = constReplace(source, config)
console.log(content);