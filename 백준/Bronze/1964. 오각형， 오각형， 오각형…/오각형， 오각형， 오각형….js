const N = +`${require('fs').readFileSync(0)}`.trim();
let dot = 5;
let inc = 7;
if (N !== 1) for (let i = 1; i < N; i++, inc += 3) dot += inc;
console.log(dot % 45678);