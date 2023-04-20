const input = `${require('fs').readFileSync(0)}`.trim();
let ans = 1;
for (let i = 0; i < input.length - 1; i++) {
  if (input[i] === input[i + 1]) {
    ans += 1;
  } else {
    break;
  }
}
console.log(ans);