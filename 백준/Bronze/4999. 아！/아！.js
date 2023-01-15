const fs = require('fs');
const [a, b] = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
console.log(a.length >= b.length ? 'go' : 'no');