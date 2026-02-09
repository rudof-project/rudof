## pg - Property Graphs

Property graphs contain a list of node or edge declarations separated by `;`.

An example can be:

```
(n1: Person { name: "Alice", age: 23 });
(n2: Person & Student { name: "Bob" });
(n3: Course { name: "Algebra" });
[e1: (n1) -[ :knows { since: 2020 }]->(n2)];
[e2: (n2)-[:knows { start: 2024, end: 2025 }]->(n3)]
```

Notice that node declarations are declared between parenthesis `(` and `)` while edge declarations are declared between square brackets `[` and `]`.

The full grammar is available [here](https://github.com/weso/pgschemapc/blob/main/src/parser/pg.rustemo).
