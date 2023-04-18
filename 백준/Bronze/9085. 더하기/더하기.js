const [N, ...i] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let j = 0; j < +N * 2; j += 2)
  ans.push(i[j + 1].split(' ').reduce((a, b) => a + +b, 0));
console.log(ans.join('\n'));