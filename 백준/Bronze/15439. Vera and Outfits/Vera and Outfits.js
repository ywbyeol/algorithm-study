const fs = require('fs');
const N = Number(fs.readFileSync('/dev/stdin', 'utf8').trim());
console.log(N * (N - 1));