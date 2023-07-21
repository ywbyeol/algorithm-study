const N = +`${require('fs').readFileSync(0)}`.trim();
console.log(N < 0 ? 32 : N.toString(2).length);