#import "../src/lib.typ": datamatrix, qrcode

#set page(width: auto, height: auto, margin: 1em, fill: luma(80%))
#set text(font: "DejaVu Sans Mono", size: 10pt, fill: luma(20%))
#set rect(inset: 0.5pt, stroke: luma(67%))

#let case(name: "", ..content) = {
  stack(
    spacing: 0.5em,
    name,
    stack(
      dir: ltr,
      spacing: 0.5em,
      ..content.pos().map(rect),
    ),
  )
}

#case(
  name: "data: \"0\"",
  datamatrix("0"),
  qrcode("0"),
)

#case(
  name: "quiet-zone: true",
  datamatrix("0", quiet-zone: true),
  qrcode("0", quiet-zone: true),
)
#case(
  name: "quiet-zone: 4 | 1",
  datamatrix("0", quiet-zone: 4),
  qrcode("0", quiet-zone: 1),
)
#case(
  name: "quiet-zone: false | none | 0",
  datamatrix("0", quiet-zone: false),
  datamatrix("0", quiet-zone: none),
  datamatrix("0", quiet-zone: 0),
)

#case(
  name: "width | height: 5em",
  datamatrix("0", width: 5em),
  datamatrix("0" * 100, width: 5em),
  datamatrix("0", height: 5em),
  datamatrix("0" * 100, height: 5em),
)
#case(
  name: "module-size: 0.1em | 0.2em",
  datamatrix("0", module-size: 0.1em),
  datamatrix("0" * 100, module-size: 0.1em),
  datamatrix("0", module-size: 0.2em),
  datamatrix("0" * 100, module-size: 0.2em),
)

#case(
  name: "fill: white | black | rainbow",
  datamatrix("0", fill: white),
  datamatrix("0", fill: black),
  datamatrix("0", fill: gradient.conic(..color.map.rainbow)),
)
#case(
  name: "background-fill: white | rainbow",
  datamatrix("0", background-fill: white),
  datamatrix("0", background-fill: gradient.conic(..color.map.rainbow)),
)
#case(
  name: "background-fill: none",
  datamatrix("0", background-fill: none),
)
#case(
  name: "background-fill + quiet-zone",
  datamatrix("0", quiet-zone: true, background-fill: white),
  datamatrix("0", quiet-zone: 4, background-fill: white),
)
#case(
  name: "background-fill + fill",
  datamatrix("0", fill: black, background-fill: white),
  datamatrix("0", fill: luma(40%), background-fill: luma(60%)),
  datamatrix(
    "0",
    fill: gradient.conic(..color.map.rainbow),
    background-fill: gradient.conic(..color.map.rainbow, angle: 180deg),
  ),
)

#case(
  name: "data: \"1\" * 1000",
  datamatrix("1" * 1000),
  qrcode("1" * 1000),
)
#case(
  name: "data: \"A\" * 1000",
  datamatrix("A" * 1000),
  qrcode("A" * 1000),
)
