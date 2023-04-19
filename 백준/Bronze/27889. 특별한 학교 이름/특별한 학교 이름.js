console.log(
  {
    NLCS: 'North London Collegiate School',
    BHA: 'Branksome Hall Asia',
    KIS: 'Korea International School',
    SJA: 'St. Johnsbury Academy',
  }[`${require('fs').readFileSync(0)}`.trim()],
);