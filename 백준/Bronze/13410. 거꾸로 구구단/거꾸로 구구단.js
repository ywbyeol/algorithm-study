const [N, K] = `${require('fs').readFileSync(0)}`.split(' ');
const arr = [];
for (let i = 1; i <= +K; i++) arr.push(+[...`${+N * i}`].reverse().join(''));
console.log(Math.max(...arr));