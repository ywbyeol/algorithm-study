const input = `${require('fs').readFileSync(0)}`.trim();
let ans = '';
for (let i = 0; i < input.length; i++) {
  let temp = parseInt(input[i], 8).toString(2);
  while (temp.length < 3 && i !== 0) temp = `0${temp}`;
  ans += temp;
}
console.log(ans);