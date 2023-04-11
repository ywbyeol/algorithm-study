const [N, ...input] = `${require('fs').readFileSync(0)}`.split('\n');
const ans = [];
for (let i = 0; i < +N; i++) {
  ans.push(
    [
  'A', 'B', 'C', 'D', 'E', 'F',
  'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R',
  'S', 'T', 'U', 'V', 'W', 'X',
  'Y', 'Z'
    ]
      .filter(x => !input[i].includes(x))
      .reduce((a, b) => a + b.charCodeAt(), 0),
  );
}
console.log(ans.join('\n'));