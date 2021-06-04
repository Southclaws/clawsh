# clawsh

A modern shell that breaks all the rules.

## Proposed syntax

The syntax ideas are still very early. The idea behind the shell is to provide as little syntax as possible and reduce the need for escape characters or quotes around "special characters". The goal is minimalism and getting out of the way of the user.

Inter-process communication is just like "normal" shells. There are no objects like PowerShell or RPCs like Nushell however there are built in pipeline tools for working with JSON output. Paired with common utilities like `ls` that output JSON or web APIs accessed with `curl` etc, the goal is to make traversing and mutating data in the shell very easy.

- `&&` conditional chain - this is in everyone's muscle memory so this stays.
- `|` pipe stdout to stdin of next command - same as above, piping commands together should be the same as other shells.
- `|{` + `}|` JS block - this is where it gets interesting, inline JS allows you to do powerful things with data.
- `|<` + `>|` jq expression - when JS is overkill, simple JQ expressions can be used in these blocks to explore data.

More syntactical tools will likely be added once these main features are done. Things like filesystem wildcards, iterating through lists (`ForEach` or `xargs` style) and more.

## Examples

List a directory, then echo only the entry names

```r
ls |{ in.map((v) => v.name)
```

List a directory, use JQ expression to parse out the names

```r
ls |< .name
```

Hit an API to get an array, extract a field from each item, hit another API using those results

```r
curl -H'Header: value' -d'{"js":["on"]}' https://api.some.site |< .value >| write file.txt
```
