const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
console.log(`${input - input * 0.22} ${input - input * 0.044}`);