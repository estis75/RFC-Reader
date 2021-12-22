# RFC-Reader

## usage

Build this program, then

```
rfc-reader LOCAL-PATH-FOR-RFC-DOCUMENT
```
will open LOCAL-PATH-FOR-RFC-DOCUMENT.
This file must be text file version of RFC.
If you don't specify any argument, then you would open the last file which you have opened.

## commands

You need to push Enter key after write each command

- :NUM 
  - move to page NUM
- \> or BLANK
  - move to next
  - if the length of \>\>...\> is n, the page would be next n page
- < 
  - move to prev
  - if the length of <<...< is n, the page would be prev n page
- q
  - quit

## supported documents

- https://www.ietf.org/rfc/ (for English)
- https://jprs.jp/tech/index.html#dns-rfc-info (for Japanese)
