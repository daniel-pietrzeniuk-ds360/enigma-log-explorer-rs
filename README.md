# Enigma+ IL log explorer
Rewritten in rust as I was obliged as a user of that language

## Download newest verison

```
https://github.com/daniel-pietrzeniuk-ds360/enigma-log-explorer-rs/raw/refs/heads/master/publish/enigma-log-explorer.exe
```

```sh
# Unix
URL=https://github.com/daniel-pietrzeniuk-ds360/enigma-log-explorer-rs/raw/refs/heads/master/publish/enigma-log-explorer.exe

curl -o enigma-log-explorer.exe $URL
```

```sh
# W*ndows
curl -o enigma-log-explorer.exe https://github.com/daniel-pietrzeniuk-ds360/enigma-log-explorer-rs/raw/refs/heads/master/publish/enigma-log-explorer.exe
```

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
