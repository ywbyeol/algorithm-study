const N = +`${require('fs').readFileSync(0)}`;
const ans = [];
for (let i = 1; i <= N; i++)
  ans.push(
    i === 1
      ? `${' '.repeat(N - i)}*`
      : i !== N
      ? `${' '.repeat(N - i)}*${' '.repeat(2 * (i - 1) - 1)}*`
      : '*'.repeat(2 * i - 1),
  );
console.log(ans.join('\n'));