process.stdin.on('data', i => {
  const [N, input] = i.toString().trim().split('\n');
  const scores = input.split(' ').map(Number);
  const M = Math.max(...scores);
  console.log(scores.reduce((a, b) => a + (b / M) * 100, 0) / Number(N));
});