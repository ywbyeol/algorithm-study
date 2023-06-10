const [T, ...S] = `${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .map(v => v.split(' '));
let [Q1, Q2, Q3, Q4, AXIS] = [0, 0, 0, 0, 0];
for (let i = 0; i < +T; i++)
  S[i].includes('0') ? AXIS++ :
    +S[i][0] > 0 ? +S[i][1] > 0 ? Q1++ : Q4++ :
    +S[i][0] < 0 ? +S[i][1] > 0 ? Q2++ : Q3++ :
    0;
console.log(`Q1: ${Q1}\nQ2: ${Q2}\nQ3: ${Q3}\nQ4: ${Q4}\nAXIS: ${AXIS}`);