# rtab

A command-line tool for generating tables from CSV.

## Examples

All of the below examples will use the following CSV file as input.

```csv
,Labial,Alveolar,Dorsal
Plosive,p,t,k
Nasal,m,n,
Fricative,,s,
Approximant,w,l,j
```

The basic style can be specified with the `--style` option, defaulting to
`basic`.

```
$ rtab input.csv
            Labial Alveolar Dorsal
Plosive     p      t        k
Nasal       m      n
Fricative          s
Approximant w      l        j

$ rtab input.csv --style fancy
┌─────────────┬────────┬──────────┬────────┐
│             │ Labial │ Alveolar │ Dorsal │
│ Plosive     │ p      │ t        │ k      │
│ Nasal       │ m      │ n        │        │
│ Fricative   │        │ s        │        │
│ Approximant │ w      │ l        │ j      │
└─────────────┴────────┴──────────┴────────┘
```

Additional spacing can be added with the `--spaces` (or `-s`) option.

```
$ rtab input.csv -s2
             Labial  Alveolar  Dorsal
Plosive      p       t         k
Nasal        m       n
Fricative            s
Approximant  w       l         j
```

The `--spaces` option works similarly for the `fancy` style.

The `fancy` style can be further customized with the `--headers` and
`--separators` flags.

```
$ rtab input.csv --style fancy --headers
┌─────────────┬────────┬──────────┬────────┐
│             │ Labial │ Alveolar │ Dorsal │
├─────────────┼────────┼──────────┼────────┤
│ Plosive     │ p      │ t        │ k      │
│ Nasal       │ m      │ n        │        │
│ Fricative   │        │ s        │        │
│ Approximant │ w      │ l        │ j      │
└─────────────┴────────┴──────────┴────────┘

$ rtab input.csv --style fancy --separators
┌─────────────┬────────┬──────────┬────────┐
│             │ Labial │ Alveolar │ Dorsal │
├─────────────┼────────┼──────────┼────────┤
│ Plosive     │ p      │ t        │ k      │
├─────────────┼────────┼──────────┼────────┤
│ Nasal       │ m      │ n        │        │
├─────────────┼────────┼──────────┼────────┤
│ Fricative   │        │ s        │        │
├─────────────┼────────┼──────────┼────────┤
│ Approximant │ w      │ l        │ j      │
└─────────────┴────────┴──────────┴────────┘
```
