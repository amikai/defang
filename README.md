# defang
```
Defang defang or fang url from FILE or standard input line by line to standard output.

Usage: defang [OPTIONS] [FILE]

Arguments:
  [FILE]  input file. if not specify, the default is stdin

Options:
  -d, --decode   fang the url (reverse operation of defang)
  -h, --help     Print help
  -V, --version  Print version
```

# Example usage

- Defang from stdin
```
❯ echo "https://www.google.com" | defang
hxxps://www[.]google[.]com
```

- Defang from file line by line
```
❯ cat foo.txt
http://www.google.com
https://www.google.com

❯ cat foo.txt | defang
hxxp://www[.]google[.]com
hxxps://www[.]google[.]com

❯ defang foo.txt
hxxp://www[.]google[.]com
hxxps://www[.]google[.]com
```

- Defang from file line by line and output to file
```
❯ cat foo.txt
http://www.google.com
https://www.google.com

❯ cat foo.txt | defang > bar.txt
❯ cat bar.txt
hxxp://www[.]google[.]com
hxxps://www[.]google[.]com

❯ defang foo.txt > bar.txt
❯ cat bar.txt
hxxp://www[.]google[.]com
hxxps://www[.]google[.]com
```

Note: You can use -d option to fang (reverse operation of defang)
