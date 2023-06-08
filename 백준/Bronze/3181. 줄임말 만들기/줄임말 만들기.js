const arr = ['i', 'pa', 'te', 'ni', 'niti', 'a', 'ali', 'nego', 'no', 'ili'];
let [F, ...I] = `${require('fs').readFileSync(0)}`.trim().split(' ');
I = I.filter(w => !arr.includes(w)).map(w => w[0].toUpperCase());
console.log(F[0].toUpperCase() + I.join(''));