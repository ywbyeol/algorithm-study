let [N, K, , ...I] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/).map(Number);
I.splice(N).forEach(i => {
  if (!((i > 0 && i < K) || (i < 0 && N + i + 1 > K))) K = i > 0 ? i - K + 1 : 2 * N - K + i + 1;
});
console.log(K);