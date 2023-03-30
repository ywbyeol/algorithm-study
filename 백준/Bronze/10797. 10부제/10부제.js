const [D, ...N] = require('fs').readFileSync('/dev/stdin', 'utf8').trim().split(/\s+/);
console.log(N.filter(x => x === D).length);