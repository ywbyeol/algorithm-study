const N = +`${require('fs').readFileSync(0)}`.trim();
const ans = [];
for (let i = 0; i < N + 1; i++) {
  const n = N - i;
  const a = n > 1 ? `${n} bottles` : n < 1 ? 'No more bottles' : `${n} bottle`;
  const b = 'of beer on the wall';
  ans.push(`${a} ${b}, ${a.toLowerCase()} of beer.
${
  n < 1
    ? `Go to the store and buy some more, ${N} bottle${N > 1 ? 's' : ''} ${b}.`
    : `Take one down and pass it around, ${
        n - 1 > 1
          ? `${n - 1} bottles`
          : n - 1 > 0
          ? `${n - 1} bottle`
          : 'no more bottles'
      } ${b}.`
}`);
}
console.log(ans.join('\n\n'));