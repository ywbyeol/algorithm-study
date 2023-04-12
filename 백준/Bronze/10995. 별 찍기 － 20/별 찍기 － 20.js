const N = +`${require('fs').readFileSync(0)}`.trim();
const str = '* '.repeat(N);
const ans = [];
for (let i = 0; i < N; i++)
  ans.push(i % 2 === 0 ? str : str.split('').reverse().join(''));
console.log(ans.join('\n'));