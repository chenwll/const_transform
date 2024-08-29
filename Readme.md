# 打包成 js 调用

```rust
$ wasm-pack build --target nodejs
```

# 使用方法

```js
npm install const-replace
```

```js
const pkg = require("const-replace");

const config = JSON.stringify({
  replaced_name: "a",
  replaced_value: [
    1,
    2,
    3,
    {
      name: "cwl",
      age: 18,
    },
  ],
});

/**
 * @description 替换常量
 * @param {string} source 源文件
 * @param {JSON} config
 */
const content = pkg.const_replace(source, config);

/**
 * @description 替换json信息
 * @param {string} source 源文件
 * @param {JSON} config
 */
const content = pkg.json_replace(source, config);
```

# 例子

参考

- ./example/index.js
- ./example/test_ts.js

npm 地址： https://www.npmjs.com/package/const-replace
