const [a, b] = `${require('fs').readFileSync(0)}`.trim().split(' ');
const ans = [];
let temp = [];
for (let i = 1; i <= +a * +b; i++) {
  temp.push(`${i}`);
  if (temp.length === +b) {
    ans.push(temp.join(' '));
    temp = [];
  }
}
console.log(ans.join('\n'));