let [N, ...a] = `${require('fs').readFileSync(0)}`.trim().split('\n');
let s = 'YES';
for (let [i, r, c] = [0, '', '']; i < +N; c = '', i++) {
  r = a[i].trim();
  for (let j = 0; j < +N; j++) c += a[j][i];
  if (r !== c) s = 'NO';
}
console.log(s);