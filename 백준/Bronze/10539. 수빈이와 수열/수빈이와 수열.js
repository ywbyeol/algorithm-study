const [T, ...B] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g);
const ans = [+B[0]];
for (let i = 1; i < +T; i++)
  ans.push(+B[i] * (i + 1) - ans.reduce((a, b) => a + +b, 0));
console.log(ans.join(' '));