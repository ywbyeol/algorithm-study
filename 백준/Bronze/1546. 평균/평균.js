process.stdin.on('data', i => {
  const [N, ...scores] = i.toString().trim().split(/\s/).map(Number);
  const M = Math.max(...scores);
  console.log(scores.reduce((a, b) => a + (b / M) * 100, 0) / Number(N));
});