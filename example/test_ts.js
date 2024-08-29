const fs = require('fs');
const pkg = require('../pkg/const_replace');
const json = fs.readFileSync('package.json');
const package = String(json);

const res = pkg.replace_json(package, JSON.stringify({
    replaced_name: 'xxx',
    replaced_value: 'const_replace'
}));

console.log(res);
