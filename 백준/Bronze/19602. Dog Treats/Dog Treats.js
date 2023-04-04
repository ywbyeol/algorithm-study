process.stdin.on('data', i => {
  const [S, M, L] = `${i}`.trim().split('\n');
  console.log(+S + 2 * +M + 3 * +L >= 10 ? 'happy' : 'sad');
});