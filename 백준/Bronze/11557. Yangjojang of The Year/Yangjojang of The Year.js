const [a, t, ...l] = [[], ...`${require('fs').readFileSync(0)}`.trim().split('\n')];
let [ms, m] = ['', 0];
for (let [i, n] = [0, +l.shift()]; i < +t; a.push(ms), i++) {
  for (let j = 0; j < n; j++) {
    const [s, c] = l.shift().split(' ');
    const nm = +c;
    if (nm > m) [ms, m] = [s, nm];
  }
}
console.log(a.join('\n'));