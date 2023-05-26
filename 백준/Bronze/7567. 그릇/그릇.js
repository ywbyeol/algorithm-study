let [pre, ...input] = `${require('fs').readFileSync(0)}`.trim();
let ans = 10;
for (let i = 0; i < input.length; i++) {
  ans += input[i] === pre ? 5 : 10;
  pre = input[i];
}
console.log(ans);