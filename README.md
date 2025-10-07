# Enigma+ IL log explorer
Rewritten in rust as I was obliged as a user of that language

## Find SessionId
When browsing session tree (in scenario view) you can right-click on html element and get `data-treenodeid` from the element source

```sql
SELECT *
FROM [EnigmaPlusGui].[dbo].[Sessions]
WHERE SessionTreeNodeId = 1663
```

## Publish

### To single file
```sh
cargo build -r
```
