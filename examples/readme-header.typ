#import "../src/lib.typ": datamatrix, qrcode

#set page(
  width: auto,
  height: auto,
  margin: 0pt,
  fill: none,
)
#let datamatrix = datamatrix.with(module-size: 5pt)
#let qrcode = qrcode.with(module-size: 5pt)

#stack(
  dir: ltr,
  spacing: 10pt,
  ..(
    datamatrix("012345"),
    datamatrix("0123456789", options: (symbols: "rect")),
    qrcode("Hello from Typst", options: (ec-level: "l")),
    qrcode("01234", options: (versions: "micro", ec-level: "l")),
  ).map(content => block(content, fill: white, inset: 10pt, radius: 5pt)),
)
