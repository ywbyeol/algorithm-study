const [w, n] = require('fs').readFileSync('/dev/stdin', 'utf8').trim().split('\n');
console.log(w[n - 1]);